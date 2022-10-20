# Generated by rust2rpm 22
%bcond_without check
%define debug_package %{nil}

%global crate anda

Name:           rust-anda
Version:        0.1.5
Release:        2%{?dist}
Summary:        Andaman Build toolchain

License:        MIT
URL:            https://crates.io/crates/anda
Source:         %{crates_source}

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
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep_online

%build
%cargo_build

%install
%cargo_install

mkdir -p %{buildroot}%{_mandir}/man1/

# install man pages
cp -v target/assets/man_pages/* %{buildroot}%{_mandir}/man1/

%changelog
%autochangelog