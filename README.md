# swallow

no_std

## A crate for things you don't care about.

Explicitly discard values. Suppress nothing else.

Installation

```
[dependencies]
swallow = "0.1"
```

Usage

```
use swallow::swallow;

// Consume and discard.
swallow(some_value);

// Evaluate and ignore.
swallow!(some_expression());

// Don't even bother moving it.
swallow!(some_existing_value);
```

Features

- No allocation.
- No dependencies.
- No unsafe code.
- No opinions.
- No questions asked.

## Sometimes the best optimization is to stop caring.

## Philosophy

Not everything deserves your attention.

Some values are computed.
Some values are returned.
Some values are simply swallowed.

swallow exists for the third category.
