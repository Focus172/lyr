
fn login(username: String, password: String) -> Option<Authenticator> {
    let mut auth = Authenticator::with_password("system-auth");
    auth.get_handler().set_credentials(username, password);
    let res = auth.authenticate();

    match res {
        Ok(_) => { Some(auth) }
        Err(_) => { None }
    }
}

fn set_env(auth: Authenticator) {
    let user = auth.get_user();
    let home = auth.get_home();
    let shell = auth.get_shell();
    let lang = auth.get_lang();
    let term = auth.get_term();

    // setenv("TERM", term ? term : "linux", 1);
    // setenv("HOME", pwd->pw_dir, 1);
    // setenv("PWD", pwd->pw_dir, 1);
    // setenv("SHELL", pwd->pw_shell, 1);
    // setenv("USER", pwd->pw_name, 1);
    // setenv("LOGNAME", pwd->pw_name, 1);
    // setenv("LANG", lang ? lang : "C", 1);

    // Set PATH if specified in the configuration
    // if let path = self.config.path {
    //     _ = setenv("PATH", config.path, 1);
    //     }
}

// enum DisplayServer {
// 	Wayland,
// 	Shell,
// 	Script,
// }

fn set_xdg_session_env(display_server: DisplayServer) {
	match display_server {
		DispayServer::Wayland => { setenv("XDG_SESSION_TYPE", "wayland", 1) },
		DispayServer::Shell => { setenv("XDG_SESSION_TYPE", "tty", 0) },
		DisplayServer::Script => {},
	}
}

// 	
// /*
// fn set_xdg_env(tty_id: u8, desktop_name: String) {
//     char user[20];
//     snprintf(user, 20, "/run/user/%d", getuid());
// 	setenv("XDG_RUNTIME_DIR", user, 0);
//     setenv("XDG_SESSION_CLASS", "user", 0);
//     setenv("XDG_SESSION_ID", "1", 0);
//     setenv("XDG_SESSION_DESKTOP", desktop_name, 0);
//     setenv("XDG_SEAT", "seat0", 0);
//     setenv("XDG_VTNR", tty_id, 0);
// }
// */
//
// void add_utmp_entry(
// 	struct utmp *entry,
// 	char *username,
// 	pid_t display_pid
// ) {
// 	entry->ut_type = USER_PROCESS;
// 	entry->ut_pid = display_pid;
// 	strcpy(entry->ut_line, ttyname(STDIN_FILENO) + strlen("/dev/"));
//
// 	/* only correct for ptys named /dev/tty[pqr][0-9a-z] */
// 	strcpy(entry->ut_id, ttyname(STDIN_FILENO) + strlen("/dev/tty"));
//
// 	time((long int *) &entry->ut_time);
//
// 	strncpy(entry->ut_user, username, UT_NAMESIZE);
// 	memset(entry->ut_host, 0, UT_HOSTSIZE);
// 	entry->ut_addr = 0;
// 	setutent();
//
// 	pututline(entry);
// }
//
// void remove_utmp_entry(struct utmp *entry) {
// 	entry->ut_type = DEAD_PROCESS;
// 	memset(entry->ut_line, 0, UT_LINESIZE);
// 	entry->ut_time = 0;
// 	memset(entry->ut_user, 0, UT_NAMESIZE);
// 	setutent();
// 	pututline(entry);
// 	endutent();
// }
//
// void xorg(
// 	struct passwd* pwd,
// 	const char* vt,
// 	const char* desktop_cmd)
// {
// 	char display_name[4];
//
// 	snprintf(display_name, 3, ":%d", get_free_display());
// 	xauth(display_name, pwd->pw_shell, pwd->pw_dir);
//
// 	// start xorg
// 	pid_t pid = fork();
//
// 	if (pid == 0)
// 	{
// 		char x_cmd[1024];
// 		snprintf(
// 			x_cmd,
// 			1024,
// 			"%s %s %s",
// 			config.x_cmd,
// 			display_name,
// 			vt);
// 		execl(pwd->pw_shell, pwd->pw_shell, "-c", x_cmd, NULL);
// 		exit(EXIT_SUCCESS);
// 	}
//
// 	int ok;
// 	xcb_connection_t* xcb;
//
// 	do
// 	{
// 		xcb = xcb_connect(NULL, NULL);
// 		ok = xcb_connection_has_error(xcb);
// 		kill(pid, 0);
// 	}
// 	while((ok != 0) && (errno != ESRCH));
//
// 	if (ok != 0)
// 	{
// 		return;
// 	}
//
// 	pid_t xorg_pid = fork();
//
// 	if (xorg_pid == 0)
// 	{
// 		char de_cmd[1024];
// 		snprintf(
// 			de_cmd,
// 			1024,
// 			"%s %s",
// 			config.x_cmd_setup,
// 			desktop_cmd);
// 		execl(pwd->pw_shell, pwd->pw_shell, "-c", de_cmd, NULL);
// 		exit(EXIT_SUCCESS);
// 	}
//
// 	int status;
// 	waitpid(xorg_pid, &status, 0);
// 	xcb_disconnect(xcb);
// 	kill(pid, 0);
//
// 	if (errno != ESRCH)
// 	{
// 		kill(pid, SIGTERM);
// 		waitpid(pid, &status, 0);
// 	}
// }
//
// void wayland(
// 	struct passwd* pwd,
// 	const char* desktop_cmd)
// {
//
// 	char cmd[1024];
// 	snprintf(cmd, 1024, "%s %s", config.wayland_cmd, desktop_cmd);
// 	execl(pwd->pw_shell, pwd->pw_shell, "-c", cmd, NULL);
// }
//
// void shell(struct passwd* pwd)
// {
// 	const char* pos = strrchr(pwd->pw_shell, '/');
// 	char args[1024];
// 	args[0] = '-';
//
// 	if (pos != NULL)
// 	{
// 		pos = pos + 1;
// 	}
// 	else
// 	{
// 		pos = pwd->pw_shell;
// 	}
//
// 	strncpy(args + 1, pos, 1023);
// 	execl(pwd->pw_shell, args, NULL);
// }
//
// // pam_do performs the pam action specified in pam_action
// // on pam_action fail, call diagnose and end pam session
// int pam_do(
// 	int (pam_action)(struct pam_handle *, int),
// 	struct pam_handle *handle,
// 	int flags,
// 	struct term_buf *buf)
// {
// 	int status = pam_action(handle, flags);
//
// 	if (status != PAM_SUCCESS) {
// 		pam_diagnose(status, buf);
// 		pam_end(handle, status);
// 	}
//
// 	return status;
// }


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
       
    // let hostname = gethostname().expect("could not get the host name");

    // return hostname
    return String::from("hostname");
}

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


// int login_conv(
// 	int num_msg,
// 	const struct pam_message** msg,
// 	struct pam_response** resp,
// 	void* appdata_ptr)
// {
// 	*resp = calloc(num_msg, sizeof (struct pam_response));
//
// 	if (*resp == NULL)
// 	{
// 		return PAM_BUF_ERR;
// 	}
//
// 	char* username;
// 	char* password;
// 	int ok = PAM_SUCCESS;
// 	int i;
//
// 	for (i = 0; i < num_msg; ++i)
// 	{
// 		switch (msg[i]->msg_style)
// 		{
// 			case PAM_PROMPT_ECHO_ON:
// 			{
// 				username = ((char**) appdata_ptr)[0];
// 				(*resp)[i].resp = strdup(username);
// 				break;
// 			}
// 			case PAM_PROMPT_ECHO_OFF:
// 			{
// 				password = ((char**) appdata_ptr)[1];
// 				(*resp)[i].resp = strdup(password);
// 				break;
// 			}
// 			case PAM_ERROR_MSG:
// 			{
// 				ok = PAM_CONV_ERR;
// 				break;
// 			}
// 		}
//
// 		if (ok != PAM_SUCCESS)
// 		{
// 			break;
// 		}
// 	}
//
// 	if (ok != PAM_SUCCESS)
// 	{
// 		for (i = 0; i < num_msg; ++i)
// 		{
// 			if ((*resp)[i].resp == NULL)
// 			{
// 				continue;
// 			}
//
// 			free((*resp)[i].resp);
// 			(*resp)[i].resp = NULL;
// 		}
//
// 		free(*resp);
// 		*resp = NULL;
// 	}
//
// 	return ok;
// }
//

