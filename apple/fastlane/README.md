fastlane documentation
----

# Installation

Make sure you have the latest version of the Xcode command line tools installed:

```sh
xcode-select --install
```

For _fastlane_ installation instructions, see [Installing _fastlane_](https://docs.fastlane.tools/#installing-fastlane)

# Available Actions

### generate

```sh
[bundle exec] fastlane generate
```

Generate project files

### lint

```sh
[bundle exec] fastlane lint
```

Lint project

### reformat

```sh
[bundle exec] fastlane reformat
```

Reformat project

### lint_formatting

```sh
[bundle exec] fastlane lint_formatting
```

Ensure project is formatted

### create_app

```sh
[bundle exec] fastlane create_app
```

Create on Developer Portal and App Store Connect

Note that this only works with a personal Apple ID in a local

session and can't be a part of CI

----


## iOS

### ios clean

```sh
[bundle exec] fastlane ios clean
```

Clean build

### ios nuke_signing

```sh
[bundle exec] fastlane ios nuke_signing
```

Nuke certs

### ios signing

```sh
[bundle exec] fastlane ios signing
```

Sync development signing

### ios test

```sh
[bundle exec] fastlane ios test
```

Test

### ios build

```sh
[bundle exec] fastlane ios build
```

Build binary incrementally

### ios incremental

```sh
[bundle exec] fastlane ios incremental
```

Runs an incremental build

### ios full

```sh
[bundle exec] fastlane ios full
```

Runs a full build for CI

### ios release

```sh
[bundle exec] fastlane ios release
```

Release binary

----

This README.md is auto-generated and will be re-generated every time [_fastlane_](https://fastlane.tools) is run.

More information about _fastlane_ can be found on [fastlane.tools](https://fastlane.tools).

The documentation of _fastlane_ can be found on [docs.fastlane.tools](https://docs.fastlane.tools).
