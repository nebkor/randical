# Radical Random Value Generator

```randical``` is a simple unix commandline utility to generate a series of
random values of varying types, with uniform distribution. See below for usage and examples.


```text
Radical Random Value Generator 1.6180

Generates arbitrary numbers of uniformly distributed random values.

USAGE:
    randical [FLAGS] [OPTIONS]

FLAGS:
        --buel       Prints either 'Here.' or 'Um, he's sick. My best friend's sister's boyfriend's brother's girlfriend
                     heard from this guy who knows this kid who's going with the girl who saw Ferris pass out at 31
                     Flavors last night. I guess it's pretty serious.', with equal probability. Not compatible with `-t`
                     or `--bule`.
        --bule       Prints either 'true' or 'false', with equal probability. Not compatible with `-t` or `--buel`.
    -e, --exit       With equal probability, exit with either status 0, like /bin/true, or status 1, like /bin/false.
                     Technically compatible with all other options, but exit status will have no relation to any
                     generated output. Sets default number of values to print to 0.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num-vals <NUM_VALS>    Number of random values to print out. Defaults to 1.
    -t, --type <TYPE>            Type of random value to print. Defaults to 'bool'.
                                 Possible values are 'b'ool, 'f'loat64, 'U'UIDv4, 'u'nsigned64, and 's'igned64
```

Some examples:

``` text
$ randical -n 5      # print out five bools
Bogus.
Radical!
Radical!
Bogus.
Radical!

$ randical -t u -n 5 # print out five 64-bit unsigned integers
5787939472744910229
3687549088276320089
5895623703396652260
1132852924593482146
15071579321211626745

$ randical -n 5 -t s # print out five 64-bit signed integers
-3655402238002064604
7349054970592683859
-4119878930309679607
3670604787450187343
7596830659839314972

$ randical -n 5 -t f # print out five 64-bit floating-point numbers in [0,1)
0.603028217883161
0.004087838255832366
0.07830762695977944
0.8930433328568959
0.6985875655193886

$ randical -n 5 -t U # print out five v4 UUIDs
0237bf92-a629-440a-a1b5-ae32741b8ebd
a88a5c2a-88c5-4b48-849b-656831eb7fc5
56ac8f8c-4e67-4418-bf0c-84a5442804c4
c82162d6-80e5-4d3e-819b-b350dba74d5e
d8194b0b-246f-4c0a-8897-cb9cdee27a99

$ randical -e         # exit with status 0 or 1, with equal probability
$ echo $?
1
$ randical -e
$ echo $?
0
$
```
