rofi_mode::export_mode!(Mode<'_>);

enum ModeState {
    Selecting,
    Confirming(String),
}

struct Mode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>,
    state: ModeState,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "logout\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("Logout");
        Ok(Self {
            api,
            entries: vec!["logout".into(), "reboot".into(), "shutdown".into()],
            state: ModeState::Selecting,
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        (&self.entries[line]).into()
    }

    // fn entry_icon(&mut self, _line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
    //     self.api.query_icon("computer", height).wait(&mut self.api)
    // }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match &self.state {
            ModeState::Selecting => match event {
                rofi_mode::Event::Cancel { selected: _ } => {
                    return rofi_mode::Action::Exit;
                }
                rofi_mode::Event::Ok {
                    alt: false,
                    selected,
                } => {
                    self.state = ModeState::Confirming(self.entries[selected].clone());
                    self.entries = vec!["yes".into(), "no".into()];
                    input.clear()
                }
                _ => {}
            },
            ModeState::Confirming(action) => match event {
                rofi_mode::Event::Cancel { selected: _ } => {
                    return rofi_mode::Action::Exit;
                }
                rofi_mode::Event::Ok {
                    alt: false,
                    selected,
                } => {
                    match self.entries[selected].as_str() {
                        "yes" => match action.as_str() {
                            "logout" => {
                                if let Ok(user) = std::env::var("USER") {
                                    let _ = std::process::Command::new("loginctl")
                                        .arg("terminate-user")
                                        .arg(user)
                                        .spawn();
                                }
                            }
                            "reboot" => {
                                let _ = std::process::Command::new("systemctl")
                                    .arg("reboot")
                                    .spawn();
                            }
                            "shutdown" => {
                                let _ = std::process::Command::new("systemctl")
                                    .arg("poweroff")
                                    .spawn();
                            }
                            _ => {}
                        },
                        "no" => {
                            self.state = ModeState::Selecting;
                            self.entries =
                                vec!["logout".into(), "reboot".into(), "shutdown".into()];
                            self.api.set_display_name("Logout");
                        }
                        _ => {}
                    };
                    input.clear();
                }
                _ => {}
            },
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line])
    }
}
