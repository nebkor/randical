# Radical Random Value Generator

```randical``` is a simple unix commandline utility to generate a series of
random values of varying types, with uniform distribution. See below for usage and examples.


```text
Radical Random Value Generator 1.618033

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
                                 Possible values are 'b'ool, 'f'loat64, 'U'UIDv4, 'u'nsigned64, 's'igned64, and 'k'suid
                                 with millisecond precision.
```

Some examples:

```text
$ randical -n 3     # print out three bools
Bogus.
Radical!
Radical!

$ randical -t u 	# 64-bit unsigned integers
5787939472744910229

$ randical -t s 	# 64-bit signed integers
-3655402238002064604

$ randical -t f 	# floating-point numbers from [0,1)
0.603028217883161

$ randical -t U 	# v4 UUIDs
0237bf92-a629-440a-a1b5-ae32741b8ebd

$ randical -e       # exit with status 0 or 1, with equal probability
$ echo $?
1
$ randical -e
$ echo $?
0
$ randical -t k -n 3 # print out three "K-sorted UID"s; see https://segment.com/blog/a-brief-history-of-the-uuid/
27wOG5VmO0vOzDsk66ELjzDa20X
27wOG5WdjugQa5HTnGr9IABktzF
27wOG5WK0MgZ49lrBUNWGe6XDFI
```

You can use it to do some simple modeling in a shell one-liners, like so:

 - simulate a fair coin toss:

```text
$ for i in {1..10} ; do (randical -e && echo heads) || echo tails ; done
heads
heads
tails
tails
heads
heads
tails
heads
heads
heads
```

 - simulate an *unfair* coin toss:

 ```text
$ for n in $(randical -n 10 -t f) ; do echo -n "unfair coin is " ; ((($n < 0.25)) && echo -n tails) || echo -n heads ; echo " ($n)" ; done
unfair coin is tails (0.08358173070875441)
unfair coin is heads (0.6536652844106665)
unfair coin is heads (0.25506794504375785)
unfair coin is heads (0.8781122125361713)
unfair coin is heads (0.3413074383739916)
unfair coin is heads (0.9068801316813913)
unfair coin is heads (0.6210935828659315)
unfair coin is heads (0.834424540129461)
unfair coin is heads (0.5440078767522896)
unfair coin is heads (0.615928198757337)
 ```

 - simulate a *Sliding Doors*-style garden of forking paths alternate timeline for Ferris Bueller's
   presence or absence on that fateful day:

```text
$ for i in {1..5} ; do echo "----" ; echo "Bueller? Bueller?" ; randical --buel ; done
----
Bueller? Bueller?
Here.
----
Bueller? Bueller?
Um, he's sick. My best friend's sister's boyfriend's brother's girlfriend heard from this guy who knows this kid who's going with the girl who saw Ferris pass out at 31 Flavors last night. I guess it's pretty serious.
----
Bueller? Bueller?
Here.
----
Bueller? Bueller?
Um, he's sick. My best friend's sister's boyfriend's brother's girlfriend heard from this guy who knows this kid who's going with the girl who saw Ferris pass out at 31 Flavors last night. I guess it's pretty serious.
----
Bueller? Bueller?
Um, he's sick. My best friend's sister's boyfriend's brother's girlfriend heard from this guy who knows this kid who's going with the girl who saw Ferris pass out at 31 Flavors last night. I guess it's pretty serious.
```
