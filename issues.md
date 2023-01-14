Note: most of my issues were with procedural nitty-gritty

# Things sufficiently covered in the getting-started docs:

- Installing wash
- Code and build a custom actor from command line
- Deploy custom actor using WebUI
- Creating custom interfaces using Smithy
- Code and build a custom provider from command line (mostly)
- Call provider from actor using custom contract (mostly)

# Things I had to figure out on my own:

- Start complete dev setup - wasmcloud host, local registry (solved)
- Install wash-cli using cargo (solved)
  - Updating wash-cli using cargo (solved)
- Capture local host ID (solved)
- Capture (cold) actor IDs (solved)
- Start actors and providers using wash (solved)
- Have to use a local registry
- Iteratively update actors and providers from command line (solved)
- Custom workflow involving clearing the wasmcloud host's OCI cache
- Call provider from actor using custom contract (partial)
- Putting link too soon after starting provider (and/or actor?) leads to "unlinked actor" error on wash call (possibly bug) (solved)
  - `sleep 1` before putting link

# Things I had to read the reference for:

- Learning the basic ideas about what actors and providers really are
- Setting the WASMCLOUD_OCI_ALLOWED_INSECURE flag for local development

# Things I need to figure out:

- Run providers on older versions of glibc
- Capture (cold) provider IDs
- Actor to actor calls in code using custom interface
