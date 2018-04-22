# envreplace

Simple tool intented to be used instead of `envsubst`.

## Why?
As stated in its `man` page:

> envsubst - substitutes environment variables in shell format strings

The problem is, variables not found in the environment are substituted with the empty string, making `envsubst` unusable with some type of configuration files (NGINX config files, for example) where it is desirable to leave the not found `$token`s as they are.

A second (personal) objective with this little tool is to learn [Rust](https://www.rust-lang.org/). It's not a big project, but is a start. There's pretty much nothing else to add without falling into ["creeping featurism"](https://en.wikipedia.org/wiki/Feature_creep).

## TODO
* Test setup / Test suite