# Generated by rust2rpm 22
%bcond_without check
%define debug_package %{nil}

%global crate anda

%global _version 0.1.5

Name:           rust-anda
Version:        %{_version}.%{autogitversion}
Release:        1%{?dist}
Summary:        Andaman Build toolchain

License:        MIT
URL:            https://crates.io/crates/anda

%global version %{_version}
Source:         https://github.com/FyraLabs/anda/archive/%{autogitcommit}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging >= 21
BuildRequires:  anda-srpm-macros
BuildRequires:  openssl-devel
BuildRequires:  git-core

Requires:       mock
Requires:       rpm-build
Requires:       createrepo_c
Requires:       git-core
%global _description %{expand:
Andaman Build toolchain.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%{_bindir}/anda
%{_mandir}/man1/anda*.1*
%prep
%autosetup -n %{crate}-%{autogitcommit} -p1
%cargo_prep_online

%build
%cargo_build
cargo xtask manpage

%install
%cargo_install

mkdir -p %{buildroot}%{_mandir}/man1/

# install man pages
cp -v target/assets/man_pages/* %{buildroot}%{_mandir}/man1/

%changelog
%autochangelog
