# Network

Network defines the p2p system of shank. It uses libp2p for its p2p stack.

The goal of this package is to provide basic functionality for a network to be bootstrapped.

Modules will be able to register a protocol and stream on the network in order to be used by the module and potentially other modules.
The base layer is interchangeable and only provides a set of basic functionality to be run.
