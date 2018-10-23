# Radical Random Value Generator

```randical``` is a simple unix commandline utility to generate a series of
random values of varying types. See below for usage and examples.



```text
USAGE:
    randical [FLAGS] [OPTIONS]

FLAGS:
    -e, --exit       Randomly exit with either status 0, like /bin/true, or status 1, like /bin/false. Technically
                     compatible with all other options, but doing so could obscure potential errors. Sets default number
                     of values to print out to 0.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num-vals <NUM_VALS>    Number of random values to print out. Defaults to 1.
    -t, --type <TYPE>            Type of random value to print. Defaults to 'bool', with true represented as '1', and
                                 false as '0'.
                                 Possible values accepted are 'b'ool, 'f'loat64, 'u'nsigned64, and 's'igned64

```

Some examples:

``` text
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
$ randical -n 10 # print out ten "bools"
0
0
0
1
0
0
1
1
1
0
$ randical -n 1 -t f -e # print out one float and exit with a status randomly true or false, in the unix exit status sense.
0.9543066009689831
$ echo $?
1
$ randical -n 1 -t f -e
0.6178924136785371
$ echo $?
0
$
```
