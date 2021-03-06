# Builder Services Development Environment

## Overview

This document captures the steps to start and run a Builder environment for development. The builder environment includes the builder services, as well as the depot web site.

There are several ways of creating a Builder dev environment - but supporting all operating systems and environments has proven to be untenable. This document includes one officially supported way of creating a Builder dev environment, and links to unsupported ways of creating the dev environment that you may use at your own risk.

## Pre-Requisites

Note that while these instructions should technically work with any linux environment that habitat supports, we recommend either a VMWare-based VM or bare metal. Other providers (e.g., Docker, VirtualBox) have posed difficulty. For instance, VirtualBox doesn't support forwarding privileged ports, which makes using the web app outside the linux environment challenging.

VMWare with Vagrant (and [the supplied Vagrantfile](https://github.com/habitat-sh/builder/blob/master/Vagrantfile)) works well, but Vagrant is not required if you're comfortable with setting up your own VM and port forwarding. For VMWare Fusion 10, adding the following lines to `/Library/Preferences/VMware Fusion/networking` seems to suffice (where 172.16.174.130 is the IP of the VM):
```
add_nat_portfwd 8 tcp 80 172.16.174.130 80 
add_nat_portfwd 8 tcp 3000 172.16.174.130 3000 
add_nat_portfwd 8 tcp 9636 172.16.174.130 9636 
```

### Ports required
1. 9636 - Intra-supervisor communication
1. 80 - Web app
1. 9631 - supervisor api port
1. 5433 - (nonstandard) posgres port (configurable in [datastore.toml](https://github.com/habitat-sh/builder/blob/master/support/builder/datastore.toml#L3))

### Checkout
* If you are developing on Linux
* * Ensure you have curl
* * `git clone https://github.com/habitat-sh/builder.git /src`
* If you are developing on a Mac
* * `git clone https://github.com/habitat-sh/builder.git`

If you are using Linux environment you can run `/src/support/linux/provision.sh` to configure your host
If you are on a Mac, you will need brew, direnv, habitat, and Docker for Mac

### GitHub OAuth Application
`APP_HOSTNAME` mentioned below, will typically be `localhost`

1. [Setup a GitHub application](https://github.com/settings/apps/new) for your GitHub organization
1. Set the value of `Homepage URL` to `http://${APP_HOSTNAME}`
1. Set the value of `User authorization callback URL` to `http://${APP_HOSTNAME}/` (The trailing `/` is *important*)
1. Set the value of `Webhook URL` to `http://${APP_HOSTNAME}/`
1. Set Repository metadata, Repository administration, Repository content and Organization members to read only (this is only used for your org so it's safe)
1. Save and download the private key. It will result in a file like `app-name.date.private-key.pem`
1. Copy the private key to `${HABITAT_SRC_ROOT}/.secrets/builder-github-app.pem`
1. Record the the client-id, client-secret, app_id and public page link (in the left sidebar). These will be used for the `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `GITHUB_APP_ID` and `GITHUB_APP_URL` build variables (seen below).

### Create app env file

1. `cp ${HABITAT_SRC_ROOT}/.secrets/habitat-env.sample ${HABITAT_SRC_ROOT}/.secrets/habitat-env`
1. Edit `${HABITAT_SRC_ROOT}/.secrets/habitat-env` with the variables from above
1. Save and close

### Studio requirements

1. Ensure you have run `hab setup` in your environment that will be executing the studio

### Starting the services

From either your VM or Mac:

* `cd <source path>`
* `direnv allow`
* `hab studio enter`
* `start-builder`

### UI Setup

If you are not doing UI development you just need to navigate to `${APP_HOSTNAME}/#/sign-in`

If you are developing the UI:

* Follow the instructions in the [Web UI README](https://github.com/habitat-sh/habitat/blob/master/components/builder-web/README.md) to get the Web UI running locally.
* Open up a browser and navigate to http://localhost:3000/#/pkgs - you should see the Habitat UI running locally.
* In the studio, you will need to run
* * `ui-dev-mode` to swap out the github application for development on `localhost:3000`
* * `upload_github_keys` to update the private key from your app with the new shared key for the app connected to `localhost:3000`
* * Note: Make sure you have copied the private key as described [here](#GitHub OAuth Application)

## Helper functions

* `start-builder` - Run the builder cluster
* `origin <name>` - Create the core origin. Default: core
* `project` - Create a project (you can also configure this in the web UI)
* `build-builder` - Build all the builder components
* `build-<component>` - Ex: `build-router` will build the router component for development and load it
* `dev_docs` - Print the full set of command docs

### Create a project using the web UI

1. Go the web UI that you used in the last step
2. Go to the origins page, and select your origin
3. Click on the 'Connect a plan file' button
4. Click on 'Install Github App' button to install the Builder Dev app on your github account
5. Go back to the Packages page (from Step 3), and follow the instructions to link the plan you want to build

Note: your GH app must have access to a repo containing a plan file. Forking `habitat-sh/core-plans` is an easy way to test.

## Run a build
`build-builder`

### Install dependencies in your local Builder env

You may use the `load_package` helper to specify a pacakge to upload. Ex:

```
load_package /hab/cache/artifacts/core-*.hart
```

If you are missing packages you can install them from the upstream Builder service. Ex:

```
hab pkg install core/elasticseach
load_package /hab/cache/artifacts/core-elasticsearch*.hart
```

### Option A: From the Web UI
* Navigate to http://${APP_HOSTNAME}/#/pkgs
* If you are not already logged in, log in.
* Click on "My origins"
* Click on "core"
* Click on the package you wish to build
* Click on "Latest"
* Click on "Build latest version"
* Click on "Build Jobs" and "View the output" to see the job in progress
* The job should complete successfully! Congrats, you have a working build!

### Option B: From the Command Line

Issue the following command (replace `core/nginx` with your origin and package names):

```
hab bldr job start core/nginx
```

This should create a build job, and then dispatch it to the build worker.

You can view the build progress in the web UI or by viewing `/hab/svc/builder-worker/data/876066265100378112/log_pipe-876066265100378112.log`. Replace `876066265100378112` with the group ID output by the `start` command.

Note: you will need to upload additional packages to the core origin for the `core/nginx` build to succeed. Follow the same procedure as for `core/hab-backline`. Currently `core/gcc` and `core/libedit` are required.
