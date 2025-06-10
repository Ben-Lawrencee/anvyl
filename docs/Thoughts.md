###### Executing modifiers
Modifiers are external 'libraries' that we must execute at runtime.

It would be great if there was a memory-safe, and very strict enforcement when executing these libraries.

The more I think about it, the more it seems that a modifier is just a DLL (Dynamically Linked Library).

Each modifier must provide a very specific interface for coupling.

Modifiers must pass a checksum and/or hash in order to be ran.
This is to prevent ACE (Arbitrary Code Execution) attacks.

Although, it will largely be the user's responsibility that they do not execute malicious code.

---
###### Hosting the mod market
This seems like it would be a very resource intensive thing to host.
Mostly on internet bandwidth, so this could be hosted in the cloud.

If hosted in the cloud, then ideally this would pay for itself in some manner.
Donations? Must pay to publish a modifier?

Pay to download?
This is not ideal as it would require authentication. Or perhaps it could just be done with an api key?
Easy for pipelines, and we can have a cli command for manual authentication.

Have users host their own?
Or at least open-source the mod market.

###### Authentication
This is a large one, and I am unsure if it is the route I want to go.