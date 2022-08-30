project "anda" {
    pre_script {
        commands = ["echo 'hello'"]
    }
    /*

    script {
        stage "build" {
            depends = ["prepare"]
            commands = [
                "cargo build --release"
                ]
        }
        stage "test" {
            depends = ["build"]
            commands = [
                "cargo test --release"
                ]
        }
    }



    docker {
        image "anda/anda" {
            version = "latest"
            dir = "."
        }
    }


    rollback {
        stage "build" {
            commands = [
                "echo 'rollback'"
            ]

        }
    }

    */

    script {
        stage "build" {
            commands = [
                "echo 'build command here'",
                "echo $TEST",
                "echo Branch: \"$BRANCH\"",
                "echo Project: $PROJECT_NAME > anda-build/build.txt",
                "ls -la"
                "echo Commit ID: $COMMIT_ID",
            ]
        }

        stage "test" {
            depends = ["build"]
            commands = [
                "ls -la anda-build",
                "echo 'test command here'",
                "echo $TEST",
                "cat anda-build/build.txt",
            ]
        }
    }

    docker {
        image "local-registry:5050/anda/anda" {
            tag_latest = true
            version = "latest"
            workdir = "."
            dockerfile = "Dockerfile"
        }
        image "172.16.5.4:5050/anda/anda-client" {
            tag_latest = true
            version = "latest"
            workdir = "."
            dockerfile = "client.dockerfile"
        }
    }

    rollback {
        stage "build" {
            commands = [
                "echo 'rollback command here'"
            ]
        }
    }

    rpmbuild {
        spec = "anda/tests/umpkg.spec"
        mode = "cargo"
        package = "anda"
        build_deps = ["openssl-devel", "rust-packaging"]
    }
    post_script {
        commands = [
            "echo 'world'"
        ]
    }

    env = {
        TEST = "test"
    }

    // if scripts are defined and type is docker or rpm, the scripts will be executed
    // before the package is built.
}

project "test" {
    script {
        stage "build" {
            image = "ubuntu:latest"
            commands = [
                "echo 'build command here'"
                "echo 'test' > anda-build/build.txt",
                "ls -la anda-build"
            ]
        }
        stage "test" {
            depends = ["build"]
            commands = [
                "echo 'test command here'",
                "cat anda-build/build.txt",
                "ls -la anda-build"
            ]
        }
    }
    rpmbuild {
        spec = "anda/tests/umpkg.spec"
    }
}