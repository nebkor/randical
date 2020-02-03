# Golden Versioning

```randical``` is versioned under a scheme I call "goldver", as an homage to the
vastly inferior [semver](https://semver.org).

## What does "goldver" mean?

When projects are versioned with goldver, the first version is "1". Note that it
is not "1.0", or, "1.0-prealpha-release-preview", or anything nonsensical like
that. As new versions are released, decimals from *phi*, the [Golden
Ratio](https://en.wikipedia.org/wiki/Golden_ratio), are appended after an
initial decimal point. So the second released version will be "1.6", the third
would be "1.61", etc., and on until perfection is asymptotically approached as
the number of released versions goes to infinity.

## Wait, didn't Donald Knuth do this?

No! He uses [pi for TeX and e for MetaFont](https://texfaq.org/FAQ-TeXfuture),
obviously COMPLETELY different.

## Ok.

Cool.

## What version is randical now?

Canonically, see the ```VERSION``` file. Heretically, now that there have been
at least three releases, the version string in the ```Cargo.toml``` file will
always be of the form "1.6.x", where *x* is at least one digit long, starting
with "1". Each subsequent release will append the next digit of *phi* to
*x*. The number of releases can be calculated by counting the number of digits
in *x* and adding 2 to that.
