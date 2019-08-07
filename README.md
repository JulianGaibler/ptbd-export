![ptbd-export](http://qvieo.com/githubimg/banner_ptbd_export.png?)

## Introduction

When I'm done with a comic, I have a bunch of pictures which need different formats, layouts and sizes depending on where I want to upload them. This tool is taking a lot of that repetitive work off my hands.

## Build Setup

The actual image compression has to be done by OpenCV since there are no (working) libraries in rust, which can do that reliably.
Maybe one day [this](https://github.com/PistonDevelopers/image/issues/862) will be fixed though.

Then take a look at `build.rs` and change the paths to opencv accordingly.

## Commits

Using [gitmoji](https://gitmoji.carloscuesta.me/) for all commits.
