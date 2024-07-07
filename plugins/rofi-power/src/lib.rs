rofi_mode::export_mode!(Mode<'_>);

enum View {
    Main,
    Confirm(String),
}

struct Mode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>,
    view: View,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "power\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("󰐥 Power");
        Ok(Self {
            api,
            entries: vec!["󰗽 Logout".into(), "󰜉 Reboot".into(), "󰤁 Shutdown".into()],
            view: View::Main,
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        (&self.entries[line]).into()
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match &self.view {
            View::Main => match event {
                rofi_mode::Event::Cancel { selected: _ } => {
                    return rofi_mode::Action::Exit;
                }
                rofi_mode::Event::Ok {
                    alt: false,
                    selected,
                } => {
                    self.view = View::Confirm(self.entries[selected].clone());
                    self.api.set_display_name(self.entries[selected].clone());
                    self.entries = vec!["Yes".into(), "No".into()];
                    input.clear()
                }
                _ => {}
            },
            View::Confirm(action) => match event {
                rofi_mode::Event::Cancel { selected: _ } => {
                    return rofi_mode::Action::Exit;
                }
                rofi_mode::Event::Ok {
                    alt: false,
                    selected,
                } => {
                    match self.entries[selected].as_str() {
                        "Yes" => match action.as_str() {
                            "Logout" => {
                                if let Ok(user) = std::env::var("USER") {
                                    let _ = std::process::Command::new("loginctl")
                                        .arg("terminate-user")
                                        .arg(user)
                                        .spawn();
                                }
                            }
                            "Reboot" => {
                                let _ = std::process::Command::new("systemctl")
                                    .arg("reboot")
                                    .spawn();
                            }
                            "Shutdown" => {
                                let _ = std::process::Command::new("systemctl")
                                    .arg("poweroff")
                                    .spawn();
                            }
                            _ => {}
                        },
                        "No" => {
                            self.view = View::Main;
                            self.entries =
                                vec!["󰗽 Logout".into(), "󰜉 Reboot".into(), "󰤁 Shutdown".into()];
                            self.api.set_display_name("󰐥 Power");
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
