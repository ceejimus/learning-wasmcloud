# Learning wasmCloud

I've fleshed out the example based on the fakepay-provider example found [here](https://wasmcloud.com/docs/app-dev/create-provider/).
I was motivated by the functional gaps in creating a provider.
The final [Testing the new provider](https://wasmcloud.com/docs/app-dev/create-provider/testing) outlined a series of steps that I used to guide my creation of this example.
In the process of development, I ran into a few issues.

Definitely all a WIP.

## Issues Encountered

Many of the issues I encountered were standard and (very) expected hiccups related to the procedural nitty-gritty of working with new technologies such as:
- installing the dev tools
- internalizing basic abstractions (actors, providers, interfaces, etc)
- interpreting error messages (I should have documented my confusions)

I haven't documented these issues as well as others as:
 1. Most were just normal growing pains 
 2. I didn't want to

Others were related to technical and conceptual gaps in the documentation and examples.
I've tried to document these well enough so that they may help to improve the docs.
My main focus here is from "Welcome" through "App Development".

I haven't read 100% of the docs (yet), so ignorance may be the cause of some of the following.

## Procedures sufficiently covered in the getting-started docs:

- Installing wash
- Adding `wasm32-unknown-unknown` rust target
- Calling actor interfaces via wash using JSON
- Coding and building custom actors from command line
- Deploying custom actors from file using WebUI
- Creating links using wash
- Unit testing actors
- Starting providers from registry using WebUI
- Creating interfaces using Smithy
- Linking to built interface libraries in actor and provider rust projects

## Procedures and insufficiently covered:

- Deploying and invoking a custom actor/provider pair using custom contracts
  - this was the main gap that motivated this
- Call provider from actor using custom contract
  - this was partially/mostly covered but lacked complete example
- Start complete dev setup - wasmcloud host, local registry
  - Using docker for local registry
- Iteratively update actors and providers from command line
  - Custom workflow involving clearing the wasmcloud host's OCI cache
  - WADM would likely make this easier, but that's a wholenother ballgame
- Installing jq dependency used in template makefiles and other examples

## Small things that helped me that may help new developers:

- Capturing local host ID using wash
- Extracting actor IDs fromn signed wasm module
- Start actors and providers using wash
  - knowing that starting these resources is not idempotent via wash
  - knowing that wasmCloud caches it's registry

## Things I had to read the reference for:

- Learning the basic ideas about what actors and providers really are
- Setting the WASMCLOUD_OCI_ALLOWED_INSECURE flag for local development

## Things I need to figure out:

- Capture provider IDs from .par files (possible?)
- Actor to actor calls in code using custom interface
  - do you need a link or does the lattice just look at interfaces/capabilities?

## Possible bugs
- Putting link too soon after starting provider (and/or actor?) leads to "unlinked actor" error on wash call (possibly bug) (solved)
  - `sleep 1` before putting link

## Sections (from the perspective of this being an article/post)
- Intro
- Setting up development environment
  - installing wash
  - installing jq
  - adding `wasm32-unknown-unknown`
  - ensuring docker installation and use of local registry (and insecure OCI flag)
  - create "up" and "down" and "setenv" scripts
- Creating "buy" actor
  - start from hello template (or just from a repo somewhere)
    - remove all unnecessary dependencies
  - create simple input output "orders" interface
  - build, push and start using make
  - calling actor using make
  - start on "deploy" script
- Create "payments" provider
  - create payments interface
  - create new provider using wash
  - building out NotImplemented stubs
  - rebuild buy actor against new interface
  - call provider from actor
- ...
- `wash-cli` reference
  - starting and stopping wasmCloud host and NATS
  - starting actors and providers from registry
  - creating links
  - explaining wasmCloud host registry cache and clearing it w/ drain
  - calling actors
- create a simple sequence diagram of solution
  - caller talks to actor via one interface which talks to provider using another
- in "creating an actor" include basic use of registry workflow:
  - `make`
  - `make push`
  - `make start`
  - `export WASMCLOUD_MY_CUSTOM_ACTOR_ID=...`
  - `wash call $WASMCLOUD_MY_CUSTOM_ACTOR_ID HttpServer.HandleRequest "{ ... }"`
- include descriptions and workarounds of the wasmCloud actor/provider registry cache problem
  - `wash drain oci`
  - `make push`
  - `wash ctl stop actor $WASMCLOUD_HOST_ID $WASMCLOUD_MY_CUSTOM_ACTOR_ID --count 0`
  - `make start`

## some ideas specific to improving current docs
- add a section about where to find logs when launching hosts using `wash up`
- add a section about host registry caching in Actor Troubleshooting
- describe more explicitly the different default `make` commands that come with the actor and provider templates
- describe how to do everything from the UI **and** CLI where both apply
  - using tabs to show/hide the sections might work?
  - I'm biased towards using CLI tools

