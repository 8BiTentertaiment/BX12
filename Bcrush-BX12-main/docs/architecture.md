# Architecture

Bcrush follows a modern graphics API design inspired by DX12 and Vulkan, including:

- Explicit resource management via Descriptor Heaps
- Command submission through Command Queues
- Modular shader and resource handling

BX12 acts as a compatibility wrapper, translating DX12 calls to Bcrush API calls.
