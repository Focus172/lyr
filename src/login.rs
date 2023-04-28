

//use"draw.h"
//use "inputs.h"

int get_free_display()
{
	char xlock[1024];
	uint8_t i;

	for (i = 0; i < 200; ++i)
	{
		snprintf(xlock, 1024, "/tmp/.X%d-lock", i);

		if (access(xlock, F_OK) == -1)
		{
			break;
		}
	}

	return i;
}

void reset_terminal(struct passwd* pwd)
{
	pid_t pid = fork();

	if (pid == 0)
	{
		execl(pwd->pw_shell, pwd->pw_shell, "-c", config.term_reset_cmd, NULL);
		exit(EXIT_SUCCESS);
	}

	int status;
	waitpid(pid, &status, 0);
}

int login_conv(
	int num_msg,
	const struct pam_message** msg,
	struct pam_response** resp,
	void* appdata_ptr)
{
	*resp = calloc(num_msg, sizeof (struct pam_response));

	if (*resp == NULL)
	{
		return PAM_BUF_ERR;
	}

	char* username;
	char* password;
	int ok = PAM_SUCCESS;
	int i;

	for (i = 0; i < num_msg; ++i)
	{
		switch (msg[i]->msg_style)
		{
			case PAM_PROMPT_ECHO_ON:
			{
				username = ((char**) appdata_ptr)[0];
				(*resp)[i].resp = strdup(username);
				break;
			}
			case PAM_PROMPT_ECHO_OFF:
			{
				password = ((char**) appdata_ptr)[1];
				(*resp)[i].resp = strdup(password);
				break;
			}
			case PAM_ERROR_MSG:
			{
				ok = PAM_CONV_ERR;
				break;
			}
		}

		if (ok != PAM_SUCCESS)
		{
			break;
		}
	}

	if (ok != PAM_SUCCESS)
	{
		for (i = 0; i < num_msg; ++i)
		{
			if ((*resp)[i].resp == NULL)
			{
				continue;
			}

			free((*resp)[i].resp);
			(*resp)[i].resp = NULL;
		}

		free(*resp);
		*resp = NULL;
	}

	return ok;
}

fn set_env(pswd: Password) {
	
	let term = std::env::get("TERM");
	let lang = std::env::get("LANG");
	
	//setenv("TERM", term ? term : "linux", 1);
	//setenv("HOME", pwd->pw_dir, 1);
	//setenv("PWD", pwd->pw_dir, 1);
	//setenv("SHELL", pwd->pw_shell, 1);
	//setenv("USER", pwd->pw_name, 1);
	//setenv("LOGNAME", pwd->pw_name, 1);
	//setenv("LANG", lang ? lang : "C", 1);

	// Set PATH if specified in the configuration
	if let path = self.config.path {
		//_ = setenv("PATH", config.path, 1);
	}
}

enum DisplayServer {
	Wayland,
	Shell,
	Script,
}

fn set_xdg_session_env(display_server: DisplayServer) {
	match display_server {
		DispayServer::Wayland => { setenv("XDG_SESSION_TYPE", "wayland", 1) },
		DispayServer::Shell => { setenv("XDG_SESSION_TYPE", "tty", 0) },
		DisplayServer::Script => {},
	}
}

	
/*
fn set_xdg_env(tty_id: u8, desktop_name: String) {
    char user[20];
    snprintf(user, 20, "/run/user/%d", getuid());
	setenv("XDG_RUNTIME_DIR", user, 0);
    setenv("XDG_SESSION_CLASS", "user", 0);
    setenv("XDG_SESSION_ID", "1", 0);
    setenv("XDG_SESSION_DESKTOP", desktop_name, 0);
    setenv("XDG_SEAT", "seat0", 0);
    setenv("XDG_VTNR", tty_id, 0);
}
*/

void add_utmp_entry(
	struct utmp *entry,
	char *username,
	pid_t display_pid
) {
	entry->ut_type = USER_PROCESS;
	entry->ut_pid = display_pid;
	strcpy(entry->ut_line, ttyname(STDIN_FILENO) + strlen("/dev/"));

	/* only correct for ptys named /dev/tty[pqr][0-9a-z] */
	strcpy(entry->ut_id, ttyname(STDIN_FILENO) + strlen("/dev/tty"));

	time((long int *) &entry->ut_time);

	strncpy(entry->ut_user, username, UT_NAMESIZE);
	memset(entry->ut_host, 0, UT_HOSTSIZE);
	entry->ut_addr = 0;
	setutent();

	pututline(entry);
}

void remove_utmp_entry(struct utmp *entry) {
	entry->ut_type = DEAD_PROCESS;
	memset(entry->ut_line, 0, UT_LINESIZE);
	entry->ut_time = 0;
	memset(entry->ut_user, 0, UT_NAMESIZE);
	setutent();
	pututline(entry);
	endutent();
}

void xauth(const char* display_name, const char* shell, char* pwd)
{
	const char* xauth_file = "lyxauth";
	char* xauth_dir = getenv("XDG_RUNTIME_DIR");
	if ((xauth_dir == NULL) || (*xauth_dir == '\0'))
	{
		xauth_dir = getenv("XDG_CONFIG_HOME");
		struct stat sb;
		if ((xauth_dir == NULL) || (*xauth_dir == '\0'))
		{
			xauth_dir = strdup(pwd);
			strcat(xauth_dir, "/.config");
			stat(xauth_dir, &sb);
			if (S_ISDIR(sb.st_mode))
			{
				strcat(xauth_dir, "/ly");
			}
			else
			{
				xauth_dir = pwd;
				xauth_file = ".lyxauth";
			}
		}
		else
		{
			strcat(xauth_dir, "/ly");
		}

		// If .config/ly/ or XDG_CONFIG_HOME/ly/ doesn't exist and can't create the directory, use pwd
		// Passing pwd beforehand is safe since stat will always evaluate false
		stat(xauth_dir, &sb);
		if (!S_ISDIR(sb.st_mode) && mkdir(xauth_dir, 0777) == -1)
		{
			xauth_dir = pwd;
			xauth_file = ".lyxauth";
		}
	}

	// trim trailing slashes
	int i = strlen(xauth_dir) - 1;
	while (xauth_dir[i] == '/') i--;
	xauth_dir[i + 1] = '\0';

	char xauthority[256];
	snprintf(xauthority, 256, "%s/%s", xauth_dir, xauth_file);
	setenv("XAUTHORITY", xauthority, 1);
	setenv("DISPLAY", display_name, 1);

	FILE* fp = fopen(xauthority, "ab+");

	if (fp != NULL)
	{
		fclose(fp);
	}

	pid_t pid = fork();

	if (pid == 0)
	{
		char cmd[1024];
		snprintf(
			cmd,
			1024,
			"%s add %s . `%s`",
			config.xauth_cmd,
			display_name,
			config.mcookie_cmd);
		execl(shell, shell, "-c", cmd, NULL);
		exit(EXIT_SUCCESS);
	}

	int status;
	waitpid(pid, &status, 0);
}

void xorg(
	struct passwd* pwd,
	const char* vt,
	const char* desktop_cmd)
{
	char display_name[4];

	snprintf(display_name, 3, ":%d", get_free_display());
	xauth(display_name, pwd->pw_shell, pwd->pw_dir);

	// start xorg
	pid_t pid = fork();

	if (pid == 0)
	{
		char x_cmd[1024];
		snprintf(
			x_cmd,
			1024,
			"%s %s %s",
			config.x_cmd,
			display_name,
			vt);
		execl(pwd->pw_shell, pwd->pw_shell, "-c", x_cmd, NULL);
		exit(EXIT_SUCCESS);
	}

	int ok;
	xcb_connection_t* xcb;

	do
	{
		xcb = xcb_connect(NULL, NULL);
		ok = xcb_connection_has_error(xcb);
		kill(pid, 0);
	}
	while((ok != 0) && (errno != ESRCH));

	if (ok != 0)
	{
		return;
	}

	pid_t xorg_pid = fork();

	if (xorg_pid == 0)
	{
		char de_cmd[1024];
		snprintf(
			de_cmd,
			1024,
			"%s %s",
			config.x_cmd_setup,
			desktop_cmd);
		execl(pwd->pw_shell, pwd->pw_shell, "-c", de_cmd, NULL);
		exit(EXIT_SUCCESS);
	}

	int status;
	waitpid(xorg_pid, &status, 0);
	xcb_disconnect(xcb);
	kill(pid, 0);

	if (errno != ESRCH)
	{
		kill(pid, SIGTERM);
		waitpid(pid, &status, 0);
	}
}

void wayland(
	struct passwd* pwd,
	const char* desktop_cmd)
{

	char cmd[1024];
	snprintf(cmd, 1024, "%s %s", config.wayland_cmd, desktop_cmd);
	execl(pwd->pw_shell, pwd->pw_shell, "-c", cmd, NULL);
}

void shell(struct passwd* pwd)
{
	const char* pos = strrchr(pwd->pw_shell, '/');
	char args[1024];
	args[0] = '-';

	if (pos != NULL)
	{
		pos = pos + 1;
	}
	else
	{
		pos = pwd->pw_shell;
	}

	strncpy(args + 1, pos, 1023);
	execl(pwd->pw_shell, args, NULL);
}

// pam_do performs the pam action specified in pam_action
// on pam_action fail, call diagnose and end pam session
int pam_do(
	int (pam_action)(struct pam_handle *, int),
	struct pam_handle *handle,
	int flags,
	struct term_buf *buf)
{
	int status = pam_action(handle, flags);

	if (status != PAM_SUCCESS) {
		pam_diagnose(status, buf);
		pam_end(handle, status);
	}

	return status;
}

void auth(
	struct desktop* desktop,
	struct text* login,
	struct text* password,
	struct term_buf* buf)
{
	int ok;

    char tty_id [3];
    snprintf(tty_id, 3, "%d", config.tty);

    // Add XDG environment variables
    env_xdg_session(desktop->display_server[desktop->cur]);
    env_xdg(tty_id, desktop->list_simple[desktop->cur]);

	// open pam session
	const char* creds[2] = {login->text, password->text};
	struct pam_conv conv = {login_conv, creds};
	struct pam_handle* handle;

	ok = pam_start(config.service_name, NULL, &conv, &handle);

	if (ok != PAM_SUCCESS)
	{
		pam_diagnose(ok, buf);
		pam_end(handle, ok);
		return;
	}

	ok = pam_do(pam_authenticate, handle, 0, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	ok = pam_do(pam_acct_mgmt, handle, 0, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	ok = pam_do(pam_setcred, handle, PAM_ESTABLISH_CRED, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	ok = pam_do(pam_open_session, handle, 0, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	// clear the credentials
	input_text_clear(password);

	// get passwd structure
	struct passwd* pwd = getpwnam(login->text);
	endpwent();

	if (pwd == NULL)
	{
		dgn_throw(DGN_PWNAM);
		pam_end(handle, ok);
		return;
	}

	// set user shell
	if (pwd->pw_shell[0] == '\0')
	{
		setusershell();

		char* shell = getusershell();

		if (shell != NULL)
		{
			strcpy(pwd->pw_shell, shell);
		}

		endusershell();
	}

	// restore regular terminal mode
	tb_clear();
	tb_present();
	tb_shutdown();

	// start desktop environment
	pid_t pid = fork();

	if (pid == 0)
	{
		// set user info
		ok = initgroups(pwd->pw_name, pwd->pw_gid);

		if (ok != 0)
		{
			dgn_throw(DGN_USER_INIT);
			exit(EXIT_FAILURE);
		}

		ok = setgid(pwd->pw_gid);

		if (ok != 0)
		{
			dgn_throw(DGN_USER_GID);
			exit(EXIT_FAILURE);
		}

		ok = setuid(pwd->pw_uid);

		if (ok != 0)
		{
			dgn_throw(DGN_USER_UID);
			exit(EXIT_FAILURE);
		}

		// get a display
		char vt[5];
		snprintf(vt, 5, "vt%d", config.tty);

		// set env (this clears the environment)
		env_init(pwd);
		// Re-add XDG environment variables from lines 508,509
		env_xdg_session(desktop->display_server[desktop->cur]);
		env_xdg(tty_id, desktop->list_simple[desktop->cur]);

		if (dgn_catch())
		{
			exit(EXIT_FAILURE);
		}

		// add pam variables
		char** env = pam_getenvlist(handle);

		for (uint16_t i = 0; env && env[i]; ++i)
		{
			putenv(env[i]);
		}

		// execute
		int ok = chdir(pwd->pw_dir);

		if (ok != 0)
		{
			dgn_throw(DGN_CHDIR);
			exit(EXIT_FAILURE);
		}

		reset_terminal(pwd);
		switch (desktop->display_server[desktop->cur])
		{
			case DS_WAYLAND:
			{
				wayland(pwd, desktop->cmd[desktop->cur]);
				break;
			}
			case DS_SHELL:
			{
				shell(pwd);
				break;
			}
			case DS_XINITRC:
			case DS_XORG:
			{
				xorg(pwd, vt, desktop->cmd[desktop->cur]);
				break;
			}
		}

		exit(EXIT_SUCCESS);
	}

	// add utmp audit
	struct utmp entry;
	add_utmp_entry(&entry, pwd->pw_name, pid);

	// wait for the session to stop
	int status;
	waitpid(pid, &status, 0);
	remove_utmp_entry(&entry);

	reset_terminal(pwd);

	// reinit termbox
	tb_init();
	tb_select_output_mode(TB_OUTPUT_NORMAL);

	// reload the desktop environment list on logout
	input_desktop_free(desktop);
	input_desktop(desktop);
	desktop_load(desktop);

	// close pam session
	ok = pam_do(pam_close_session, handle, 0, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	ok = pam_do(pam_setcred, handle, PAM_DELETE_CRED, buf);

	if (ok != PAM_SUCCESS)
	{
		return;
	}

	ok = pam_end(handle, 0);

	if (ok != PAM_SUCCESS)
	{
		pam_diagnose(ok, buf);
	}
}

enum DGN_ERROR {
	DGN_OK, // do not remove
	DGN_NULL,
	DGN_ALLOC,
	DGN_BOUNDS,
	DGN_DOMAIN,
	DGN_MLOCK,
	DGN_XSESSIONS_DIR,
	DGN_XSESSIONS_OPEN,
	DGN_PATH,
	DGN_CHDIR,
	DGN_PWNAM,
	DGN_USER_INIT,
	DGN_USER_GID,
	DGN_USER_UID,
	DGN_PAM,
	DGN_HOSTNAME,
	DGN_SIZE, // do not remove
}


// void desktop_load(struct desktop* target);
// void hostname(char** out);
// void free_hostname();
// void switch_tty(struct term_buf* buf);
// void save(struct desktop* desktop, struct text* login);
// void load(struct desktop* desktop, struct text* login);




// void desktop_crawl(
// 	struct desktop* target,
// 	char* sessions,
// 	enum display_server server)
// {
// 	DIR* dir;
// 	struct dirent* dir_info;
// 	int ok;
//
// 	ok = access(sessions, F_OK);
//
// 	if (ok == -1)
// 	{
// 		dgn_throw(DGN_XSESSIONS_DIR);
// 		return;
// 	}
//
// 	dir = opendir(sessions);
//
// 	if (dir == NULL)
// 	{
// 		dgn_throw(DGN_XSESSIONS_OPEN);
// 		return;
// 	}
//
// 	char* name = NULL;
// 	char* exec = NULL;
//
// 	struct configator_param map_desktop[] =
// 	{
// 		{"Exec", &exec, config_handle_str},
// 		{"Name", &name, config_handle_str},
// 	};
//
// 	struct configator_param* map[] =
// 	{
// 		NULL,
// 		map_desktop,
// 	};
//
// 	struct configator_param sections[] =	
// 	{
// 		{"Desktop Entry", NULL, NULL},
// 	};
//
// 	uint16_t map_len[] = {0, 2};
// 	uint16_t sections_len = 1;
//
// 	struct configator desktop_config;
// 	desktop_config.map = map;
// 	desktop_config.map_len = map_len;
// 	desktop_config.sections = sections;
// 	desktop_config.sections_len = sections_len;
//
// #if defined(NAME_MAX)
// 	char path[NAME_MAX];
// #elif defined(_POSIX_PATH_MAX)
// 	char path[_POSIX_PATH_MAX];
// #else
// 	char path[1024];
// #endif
//
// 	dir_info = readdir(dir);
//
// 	while (dir_info != NULL)
// 	{
// 		if ((dir_info->d_name)[0] == '.')
// 		{
// 			dir_info = readdir(dir);
// 			continue;
// 		}
//
// 		snprintf(path, (sizeof (path)) - 1, "%s/", sessions);
// 		strncat(path, dir_info->d_name, (sizeof (path)) - 1);
// 		configator(&desktop_config, path);
//
// 		// if these are wayland sessions, add " (Wayland)" to their names,
// 		// as long as their names don't already contain that string
// 		if (server == DS_WAYLAND && config.wayland_specifier)
// 		{
// 			const char wayland_specifier[] = " (Wayland)";
// 			if (strstr(name, wayland_specifier) == NULL)
// 			{
// 				name = realloc(name, (strlen(name) + sizeof(wayland_specifier) + 1));
// 				// using strcat is safe because the string is constant
// 				strcat(name, wayland_specifier);
// 			}
// 		}
//
// 		if ((name != NULL) && (exec != NULL))
// 		{
// 			input_desktop_add(target, name, exec, server);
// 		}
//
// 		name = NULL;
// 		exec = NULL;
// 		dir_info = readdir(dir);
// 	}
//
// 	closedir(dir);
// }

// void desktop_load(struct desktop* target)
// {
// 	// we don't care about desktop environments presence
// 	// because the fallback shell is always available
// 	// so we just dismiss any "throw" for now
// 	int err = 0;
//
// 	desktop_crawl(target, config.waylandsessions, DS_WAYLAND);
//
// 	if (dgn_catch())
// 	{
// 		++err;
// 		dgn_reset();
// 	}
//
// 	desktop_crawl(target, config.xsessions, DS_XORG);
//
// 	if (dgn_catch())
// 	{
// 		++err;
// 		dgn_reset();
// 	}
// }


fn hostname() -> String {
    // TODO: implement some sort of cacheing for hostname and check if that is not NULL



    // get the var name _SC_HOST_NAME_MAX
    //
    // if it is less than zero or not assainged then get _POSIX_HOST_NAME_MAX (64)
       
    let hostname = gethostname().expect("could not get the host name");

    return hostname
    // return "".to_string();
}

// void switch_tty(struct term_buf* buf)
// {
// 	FILE* console = fopen(config.console_dev, "w");
//
// 	if (console == NULL)
// 	{
// 		buf->info_line = lang.err_console_dev;
// 		return;
// 	}
//
// 	int fd = fileno(console);
//
// 	ioctl(fd, VT_ACTIVATE, config.tty);
// 	ioctl(fd, VT_WAITACTIVE, config.tty);
//
// 	fclose(console);
// }

// void save(struct desktop* desktop, struct text* login)
// {
// 	if (config.save)
// 	{
// 		FILE* fp = fopen(config.save_file, "wb+");
//
// 		if (fp != NULL)
// 		{
// 			fprintf(fp, "%s\n%d", login->text, desktop->cur);
// 			fclose(fp);
// 		}
// 	}
// }

// void load(struct desktop* desktop, struct text* login)
// {
// 	if (!config.load)
// 	{
// 		return;
// 	}
//
// 	FILE* fp = fopen(config.save_file, "rb");
//
// 	if (fp == NULL)
// 	{
// 		return;
// 	}
//
// 	char* line = malloc(config.max_login_len + 1);
//
// 	if (line == NULL)
// 	{
// 		fclose(fp);
// 		return;
// 	}

// 	if (fgets(line, config.max_login_len + 1, fp))
// 	{
// 		int len = strlen(line);
// 		strncpy(login->text, line, login->len);
//
// 		if (len == 0)
// 		{
// 			login->end = login->text;
// 		}
// 		else
// 		{
// 			login->end = login->text + len - 1;
// 			login->text[len - 1] = '\0';
// 		}
// 	}
// 	else
// 	{
// 		fclose(fp);
// 		free(line);
// 		return;
// 	}
//
// 	if (fgets(line, config.max_login_len + 1, fp))
// 	{
// 		int saved_cur = abs(atoi(line));
//
// 		if (saved_cur < desktop->len)
// 		{
// 			desktop->cur = saved_cur;
// 		}
// 	}
//
// 	fclose(fp);
// 	free(line);
// }
