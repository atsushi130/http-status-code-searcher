#!/bin/bash

: "Release to crate.io" && {
    cargo package
    cargo publish
}