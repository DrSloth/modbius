The name is obviously not final.

Generally the library is aimed to deliver allocation free reading of modbus sources.
This can easily be done in multiple ways and it has to be explored what the best way to achieve this is.

This library is likely split into multiple crates where one is optimally no alloc and no std. 
This crate just parses requests and handles raw modbus byte streams.

The other then builds upon that and provides async reading from an RTU bus or a TCP stream.

For read requests:
Returning the read content is out of question as the only real way to do this would be a Vector and vectors allocate on the heap
which we want to prevent. Allocations are often slow and providing controll to the caller is what we want.

The first question is wether quantity should be a seperate parameter or not, my tendency is no.
The reasoning behind that is that a buffer could be insufficient for the requested quantity.
If a buffer is too small the other data has to be returned or stored through other means.

- Option 1: Allocate a vector for the rest of the data, if the buffer is sufficiently large the array will be unallocated and empty
            which doesn't present real overhead (besides a stack write of 3 usizes (wow)) the buffer will be correct more often then not
- Option 2: Return instances of AsyncRead/Read/Adapted versions of that after calling the functions. This has substantial complication overhead
            this may also result in performance overhead, need for quirky lifetimes and also requires more generics

Defining the quantity to read as the len of the slice is easy enough and provides the most controll to the callee. T
This is the likeliest thing to provide allocation freedom, there could also be read_quantity functions which take &mut Vec.

The more low level parser library would then parse requests and handle everything else that can be done with a modbus byte stream.
However as this library doesn't handle the requests it can't handle the size requirements, but it doesn't have to because it just 
wants to stay at the low level it can just have one panicking/erroring and one unsafe function.

Another questions is how modbus byte slices are passed between functions. There are three options

- Option 1: Always pass the complete slice. The only functions that can't do that are the ones which handle frames
- Option 2: Always also return the next starting index.
- Option 3: Always return the slice to keep working with, this is the most flexible approach, however i hope there are no lifetime conflicts

One important design decision is how parser functions should propagate a buffer being too small. 
The easiest thing would be propagating as error and just not worrying about it at all.
Also _unchecked function of parsers where data is expected to be left require data to be left while checked ones return option for them individually.
