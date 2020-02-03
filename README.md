# Radical Random Value Generator

```randical``` is a simple unix commandline utility to generate a series of
random values of varying types, with uniform distribution. See below for usage and examples.


```text
Radical Random Value Generator 1.61
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
    -t, --type <TYPE>            Type of random value to print. Defaults to 'bool', with true represented as 'Radical!',
                                 and false as 'Bogus.'.
                                 Possible values are 'b'ool, 'f'loat64, 'u'nsigned64, and 's'igned64
```

Some examples:

``` text
$ randical -n 10      # print out ten bools
Bogus.
Radical!
Radical!
Bogus.
Radical!
Bogus.
Radical!
Bogus.
Radical!
Radical!

$ randical -t u -n 10 # print out ten 64-bit unsigned integers
5787939472744910229
3687549088276320089
5895623703396652260
1132852924593482146
15071579321211626745
17449511910217057014
15100162199599245434
16771457972349018485
7609614558571403402
8284410620633392032

$ randical -n 10 -t s # print out ten 64-bit signed integers
-3655402238002064604
7349054970592683859
-4119878930309679607
3670604787450187343
7596830659839314972
-3642333771475302770
2921931257318542851
-4580256882393100929
3009966650832330749
6676004827997477043

$ randical -n 10 -t f # print out ten 64-bit floating-point numbers in [0,1)
0.603028217883161
0.004087838255832366
0.07830762695977944
0.8930433328568959
0.6985875655193886
0.8088176723597311
0.747504385125212
0.4487145473864015
0.3171660044903156
0.29296569910381276

$ randical -e         # exit with status 0 or 1, with equal probability
$ echo $?
1
$ randical -e
$ echo $?
0
$
```
