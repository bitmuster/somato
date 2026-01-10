

![CI-Github](https://github.com/bitmuster/somato/actions/workflows/ci.yml/badge.svg)

Somato: The Solawi Management Tool
==================================

This project is a simple validator for some of the vegetable distribution
mechanics for the Soawi Heckengäu ( https://solawiheckengaeu.de/ ).

Before changing the current distribution mechanics we decided first to
write a validator that checks the correctness of the validation procedures.

Changing the mechanics became necessary as the current systems turned out
to be very hard to maintain and to adopt to new use cases.
Multiple layers of Excel power queries that spread multiple tables turned
out to be hard to maintain and quality assurance almost impossible.

Therefore, this project places high expectations into quality assurance and
software testing.


To-Do
=====

* Currently not possible to detect: Two persons with same initals at the same
    location. We might see an error in the log, but no clear indication.

Testing
=======

This project uses injectorpp to facilitate run-time mocks.
There seems to be a race condition between mocks when tests run in multiple
threads. As workaround, we run the tests in one thread:

    cargo test -- --test-threads=1
