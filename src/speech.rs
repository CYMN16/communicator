//! Text-to-speech.
//!
//! Without speech output the app is a point board: the listener has to be
//! looking at the screen. With it, the user can address someone across the
//! room, or someone whose back is turned.
//!
//! The web build uses the browser's `SpeechSynthesis`. The native build drives
//! whichever system speech binary is installed (`spd-say`, `espeak-ng`, `say`,
//! `PowerShell`) rather than linking a speech library, so the app keeps building
//! for every target in CI. [`Voice::available`] tells the UI whether to promise
//! anything.

/// A speech engine, or a truthful admission that there isn't one.
#[derive(Default)]
pub struct Voice {
    #[cfg(not(target_arch = "wasm32"))]
    backend: Option<native::Backend>,
    #[cfg(not(target_arch = "wasm32"))]
    speaking: std::sync::Mutex<Option<std::process::Child>>,
}

impl Voice {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Self {
            backend: native::detect(),
            speaking: std::sync::Mutex::new(None),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self {}
    }

    /// Is there anything on this device that can actually talk?
    #[cfg(not(target_arch = "wasm32"))]
    pub fn available(&self) -> bool {
        self.backend.is_some()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn available(&self) -> bool {
        web::synthesis().is_some()
    }

    /// Speak `text`, cancelling anything already in progress.
    ///
    /// `lang` is a BCP-47 tag such as `en-US`; `rate` is a multiplier where
    /// 1.0 is the engine's normal speed.
    pub fn speak(&self, text: &str, lang: &str, rate: f32) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        self.stop();

        #[cfg(target_arch = "wasm32")]
        web::speak(text, lang, rate);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(backend) = self.backend {
            match backend.spawn(text, lang, rate) {
                Ok(child) => {
                    if let Ok(mut slot) = self.speaking.lock() {
                        *slot = Some(child);
                    }
                }
                Err(err) => log::warn!("speech failed: {err}"),
            }
        }
    }

    /// Stop mid-utterance — needed the moment a user says the wrong thing.
    pub fn stop(&self) {
        #[cfg(target_arch = "wasm32")]
        web::cancel();

        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(mut slot) = self.speaking.lock() {
            if let Some(mut child) = slot.take() {
                child.kill().ok();
                child.wait().ok();
            }
        }
    }
}

// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web {
    pub fn synthesis() -> Option<web_sys::SpeechSynthesis> {
        web_sys::window()?.speech_synthesis().ok()
    }

    pub fn speak(text: &str, lang: &str, rate: f32) {
        let Some(synth) = synthesis() else {
            return;
        };
        match web_sys::SpeechSynthesisUtterance::new_with_text(text) {
            Ok(utterance) => {
                utterance.set_lang(lang);
                utterance.set_rate(rate);
                synth.speak(&utterance);
            }
            Err(err) => log::warn!("speech failed: {err:?}"),
        }
    }

    pub fn cancel() {
        if let Some(synth) = synthesis() {
            synth.cancel();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::io;
    use std::process::{Child, Command, Stdio};

    /// The system speech commands we know how to drive, most capable first.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Backend {
        /// Linux desktops running speech-dispatcher.
        SpdSay,
        EspeakNg,
        Espeak,
        /// macOS, always present.
        Say,
        /// Windows, via `System.Speech`.
        PowerShell,
    }

    impl Backend {
        const ALL: [Self; 5] = [
            Self::SpdSay,
            Self::EspeakNg,
            Self::Espeak,
            Self::Say,
            Self::PowerShell,
        ];

        fn program(self) -> &'static str {
            match self {
                Self::SpdSay => "spd-say",
                Self::EspeakNg => "espeak-ng",
                Self::Espeak => "espeak",
                Self::Say => "say",
                Self::PowerShell => "powershell",
            }
        }

        /// `lang` arrives as BCP-47 (`tr-TR`); most engines want just `tr`.
        pub fn spawn(self, text: &str, lang: &str, rate: f32) -> io::Result<Child> {
            let short = lang.split('-').next().unwrap_or("en");
            let mut command = Command::new(self.program());
            match self {
                Self::SpdSay => {
                    // -r takes -100..100; map 0.6..1.4 onto roughly that range.
                    let rate = ((rate - 1.0) * 125.0).clamp(-100.0, 100.0) as i32;
                    command.args(["-l", short, "-r", &rate.to_string(), "-w", "--", text]);
                }
                Self::EspeakNg | Self::Espeak => {
                    let words_per_minute = (175.0 * rate).clamp(80.0, 450.0) as i32;
                    command.args(["-v", short, "-s", &words_per_minute.to_string(), "--", text]);
                }
                Self::Say => {
                    let words_per_minute = (180.0 * rate).clamp(90.0, 500.0) as i32;
                    command.args(["-r", &words_per_minute.to_string(), "--", text]);
                }
                Self::PowerShell => {
                    let escaped = text.replace('\'', "''");
                    command.args([
                        "-NoProfile",
                        "-Command",
                        &format!(
                            "Add-Type -AssemblyName System.Speech; \
                             $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                             $s.Rate = {}; $s.Speak('{escaped}')",
                            ((rate - 1.0) * 10.0).clamp(-10.0, 10.0) as i32
                        ),
                    ]);
                }
            }
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
    }

    /// Probe for a usable engine once, at start-up.
    pub fn detect() -> Option<Backend> {
        Backend::ALL.into_iter().find(|backend| {
            Command::new(backend.program())
                .arg(if *backend == Backend::PowerShell {
                    "-Help"
                } else {
                    "--version"
                })
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok()
        })
    }
}
