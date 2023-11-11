# Complogic

[![Tests](https://github.com/Vandesm14/complogic/actions/workflows/tests.yml/badge.svg)](https://github.com/Vandesm14/complogic/actions/workflows/tests.yml)
[![Checks](https://github.com/Vandesm14/complogic/actions/workflows/check.yml/badge.svg)](https://github.com/Vandesm14/complogic/actions/workflows/check.yml)

Simulate logic gates on a compiled single-instruction VM with Rust.

## Why?

I wanted to build a system to work with a known-system rather than interpreting the circuit graph each tick, even if it was lazy. With Complogic, all gates are compiled into a set of Nand instructions, which execute at runtime.

## What?

With this design decision comes the power to ignore propagation entirely and set everything all at once. There isn't any "run until things settle" logic, and the system is entirely deterministic.

## How?

The VM only cares about outputs. Inputs are immediates, given when running the simulation. Any gate that needs an input, can either use the output of another gate, or the immediates. This allows everything to be incredibly dense, inside the register stack of the VM.
