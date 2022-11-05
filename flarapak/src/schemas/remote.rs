use configparser::ini::Ini;
use serde::Serialize;
use serde_derive::Deserialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Remote {
    pub url: String,
    pub xa: Option<Xa>,
    pub gpg_verify: Option<bool>,
    pub gpg_verify_summary: Option<bool>,
    pub name: String,
}

#[derive(Deserialize, Clone, Serialize, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Xa {
    pub prio: Option<usize>,
    pub noenumerate: Option<bool>,
    pub main_ref: Option<String>,
    pub pinned: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub title_is_set: Option<bool>,
    pub description_is_set: Option<bool>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub homepage: Option<String>,
}
pub fn deserialize(ini: Ini) -> Vec<Remote> {
    let mut remotes = Vec::new();
    for section in ini.sections() {
        if section.starts_with("remote ") {
            remotes.push(Remote {
                url: ini.get(&section, "url").unwrap(),
                xa: Some(Xa {
                    title: ini.get(&section, "xa.title"),
                    prio: ini
                        .getuint(&section, "xa.prio")
                        .unwrap()
                        .map(|int| int as usize),
                    noenumerate: ini.getbool(&section, "xa.noenumerate").unwrap(),
                    main_ref: ini.get(&section, "xa.main-ref"),
                    pinned: ini.get(&section, "xa.pinned"),
                    description: ini.get(&section, "xa.description"),
                    title_is_set: ini.getbool(&section, "xa.title-is-set").unwrap(),
                    description_is_set: ini.getbool(&section, "xa.description-is-set").unwrap(),
                    comment: ini.get(&section, "xa.comment"),
                    icon: ini.get(&section, "xa.icon"),
                    homepage: ini.get(&section, "xa.homepage"),
                }),
                gpg_verify: ini.getbool(&section, "gpg-verify").unwrap(),
                gpg_verify_summary: ini.getbool(&section, "gpg-verify-summary").unwrap(),
                name: section
                    .strip_prefix("remote \"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string(),
            })
        }
    }

    remotes
}

#[cfg(test)]
mod test {
    use configparser::ini::Ini;

    use crate::schemas::remote::{Remote, Xa};

    use super::deserialize;

    #[test]
    pub fn parse_remotes() {
        let mut ini = Ini::new();
        ini.read(include_str!("flatpak_repoconfig.ini").to_string())
            .unwrap();

        assert_eq!(
            deserialize(ini),
            vec![Remote {
                url: "https://dl.flathub.org/repo/".to_string(),
                xa: Some(Xa {
                    prio: None,
                    noenumerate: None,
                    main_ref: None,
                    pinned: None,
                    title: Some("Fedora Flathub Selection".to_string()),
                    description: Some(
                        "Selected applications from Flathub (https://flathub.org)".to_string()
                    ),
                    title_is_set: Some(true),
                    description_is_set: Some(true),
                    comment: Some("Selected applications from Flathub".to_string()),
                    icon: Some("https://dl.flathub.org/repo/logo.svg".to_string()),
                    homepage: Some("https://flathub.org/".to_string())
                }),
                gpg_verify: Some(true),
                gpg_verify_summary: Some(true),
                name: "flathub".to_string()
            }]
        );
    }
}
