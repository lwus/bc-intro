
# Social Recovery Program Specification

# Objective

In this project, you will design and implement a mechanism that allows a user
to recover a wallet’s ownership through social recovery. You will create a
program that manages a program wallet per user that allows the authorized user
to perform signing transactions with it.

For example, if Alice is the authority on program wallet Alpha, and Alpha has 5
USDC tokens, Alice should be able to instruct Alpha to transfer, burn, etc,
those tokens with her externally owned signature.

In addition, the program should manage an additional list of keys per program
wallet that allow a 'social recovery' of the authorized user (aka if Alice
loses her key that authorizes Alpha, then we can 'recover' her access, with the
approval of that additional list, by assigning the authorized user to a new
public key). To initiate the recovery, at least T of the N keys in the
additional list must provide signatures.


## Instructions

This project is much more open ended in terms of design and instruction feature
set. Minimally, these are the instructions you should plan to implement.

### Instruction 0: InitializeSocialWallet

This instruction should allow a user to initialize a socially-recoverable
program wallet. Consider how users might find this program wallet and if they
can initialize it on behalf of others.

### Instruction 1: SetRecovery

This instruction should allow the program wallet authority the ability to
add, modify, and delete public keys to the the recovery list, and manage the
recovery threshold (how many of the N addresses are required to recover the
authority) or other critieria.

This could be one or several instructions.

### Instruction 2: RecoverWallet

This instruction should allow a quorum of the recovery wallets to change the
authority of the program wallet. Ideally this is implemented in a way that
allows a single transaction to update the authority, but consider what this
means with respect to round-trip between the recovery keys (hint: durable
nonce) and if we might run into other runtime limitiations (transaction size,
compute, etc).

### Instruction 3: ApplyTransaction

Minimally, we would like to be able to transfer native SOL and SPL tokens from
the program wallet.

More generally, we are interested in arbitrary CPIs to other programs where the
program wallet is one of the signers.


## Design considerations

You are not required to implement a front-end or CLI or others. But consider in
your design the impacts of your account layout for indexers, wallet adapters,
and users in general. Is there significant overhead in using your program
wallet? Is there a way for people to find their program wallets? How might one
consolidate program wallets if you can have multiple?


