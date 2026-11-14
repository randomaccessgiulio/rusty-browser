Name:           rusty-browser
Version:        0.2.8
Release:        1%{?dist}
Summary:        Minimal desktop web browser written in Rust
License:        LicenseRef-Unknown
URL:            https://github.com/randomaccessgiulio/rusty-browser
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gcc
BuildRequires:  libgtk-3.so.0()(64bit)
BuildRequires:  libwebkit2gtk-4.1.so.0()(64bit)
Requires:       curl

%description
Rusty Browser is a minimal GTK desktop browser written in Rust and powered by
WebKitGTK. It supports modern web pages, JavaScript, and temporary sessions.

%prep
%setup -q

%build
cargo build --release --offline

%install
install -D -m 0755 target/release/rusty-browser \
    %{buildroot}%{_bindir}/rusty-browser
install -D -m 0644 packaging/rusty-browser.desktop \
    %{buildroot}%{_datadir}/applications/rusty-browser.desktop
install -D -m 0644 assets/rusty-browser.svg \
    %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/rusty-browser.svg
install -D -m 0644 assets/rusty-browser.png \
    %{buildroot}%{_datadir}/icons/hicolor/512x512/apps/rusty-browser.png
install -D -m 0644 README.md \
    %{buildroot}%{_docdir}/%{name}/README.md
install -m 0644 CHANGELOG.md CREDITS.md \
    %{buildroot}%{_docdir}/%{name}/

%files
%{_bindir}/rusty-browser
%{_datadir}/applications/rusty-browser.desktop
%{_datadir}/icons/hicolor/scalable/apps/rusty-browser.svg
%{_datadir}/icons/hicolor/512x512/apps/rusty-browser.png
%doc %{_docdir}/%{name}/README.md
%doc %{_docdir}/%{name}/CHANGELOG.md
%doc %{_docdir}/%{name}/CREDITS.md

%changelog
* Sat Nov 14 2026 Giulio <itsgiuliooo@users.noreply.github.com> - 0.2.8-1
- Add RPM package for Fedora and openSUSE Tumbleweed
