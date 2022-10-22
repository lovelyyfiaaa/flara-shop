/**
 * [core]
repo_version=1
mode=bare-user-only
min-free-space-size=500MB
xa.pinned=runtime/org.fedoraproject.Platform/x86_64/f35;runtime/org.freedesktop.Sdk.Extension.llvm12/x86_64/21.08;runtime/org.freedesktop.Sdk.Extension.node14/x86_64/21.08;runtime/org.freedesktop.Sdk.Extension.rust-stable/x86_64/21.08;runtime/org.gnome.Platform/x86_64/master;runtime/org.gnome.Sdk/x86_64/42;runtime/org.gnome.Sdk/x86_64/master;runtime/org.gtk.Gtk3theme.adw-gtk3-dark/x86_64/3.22

[remote "fedora"]
url=oci+https://registry.fedoraproject.org
gpg-verify=true
gpg-verify-summary=true

[remote "fedora-testing"]
url=oci+https://registry.fedoraproject.org#testing
xa.title=Fedora Flatpaks (testing)
xa.title-is-set=true
xa.disable=true

[remote "flathub"]
url=https://dl.flathub.org/repo/
xa.title=Fedora Flathub Selection
gpg-verify=true
gpg-verify-summary=true
xa.comment=Selected applications from Flathub
xa.description=Selected applications from Flathub (https://flathub.org)
xa.icon=https://dl.flathub.org/repo/logo.svg
xa.homepage=https://flathub.org/
xa.title-is-set=true
xa.comment-is-set=true
xa.description-is-set=true

[remote "gnome-nightly"]
url=https://nightly.gnome.org/repo/
xa.title=GNOME Nightly
gpg-verify=true
gpg-verify-summary=true
xa.description=The latest GNOME flatpak runtimes/apps directly from the gitlab CI. This is highly unstable stuff aimed at GNOME developers/designers to test
xa.homepage=https://gnome.org/
xa.gpg-keys-hash=18651a8e0151750586051c872d38670d30fbc7382ecf93d90a7bd4df6b9b803e

[remote "flathub-beta"]
url=https://dl.flathub.org/beta-repo/
xa.title=Flathub beta
gpg-verify=true
gpg-verify-summary=true
xa.comment=Beta builds of Flatpak applications
xa.description=Beta builds of Flatpak applications
xa.icon=https://dl.flathub.org/repo/logo.svg
xa.homepage=https://flathub.org/

[remote "devel-origin"]
url=
xa.title=pods.flatpak
xa.noenumerate=true
xa.prio=0
gpg-verify-summary=false
gpg-verify=false
xa.main-ref=app/com.github.marhkb.Pods.Devel/x86_64/master

 */
use serde_derive::Deserialize;
#[derive(Deserialize)]
pub struct Remote {
    pub url: String,
    pub xa: Option<Xa>,
    pub gpg_verify: Option<bool>,
    pub gpg_verify_summary: Option<bool>,
}

#[derive(Deserialize)]
pub struct Xa {
    pub prio: Option<usize>,
    pub noenumerate: Option<bool>,
    pub main_ref: Option<bool>,
    pub pinned: Option<String>
}
