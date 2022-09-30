# Generated by rust2rpm 22
%bcond_without check
%define debug_package %{nil}

%global crate anda

Name:           rust-anda
Version:        0.1.0
Release:        %autorelease
Summary:        Andaman Build toolchain

License:        MIT
URL:            https://crates.io/crates/anda
Source:         %{crates_source}

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging >= 21
BuildRequires:  fyra-srpm-macros

%global _description %{expand:
Andaman Build toolchain.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
# FIXME: no license files detected
%{_bindir}/anda

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep_online

%build
%cargo_build

%install
%cargo_install

%changelog
%autochangelog
