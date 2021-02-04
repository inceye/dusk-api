This document contains the full description of the plugin interface: how it's built, why it is built like this and how to use it in the most effective way possible.
============

If you do not care about the api internals and are only here to find out how to use it, just skip to the part #3.

**#1 Library loading**

------------

The whole point of the API is to define a common interface, which a plugin may implement and pass it into another program while it's working dynamically.

But there are several problems with dynamic library loading, which just make our life a little bit harder. The first problem is that you only can export statics, which basically just means that we can only export static variables and static functions, which is not enough for a working plugin. The second problem is that we always have to track rust version and api versions, the plugin was compiled with, as if they do not match with versions the main program uses, they may be incompatible.

The second problem is resolved trivially by just including the version checks into the loading function and version numbers into the exported static variable by forcing plugin writers to only export their plugins with a macro which does it right.

The first problem though is much more complicated. Basically, if we need to get the ability to export any amount of functions with some more info about them easily, we have to do it by defining a common behavior in a trait and then somehow export a structure that implements this trait into the program, that is importing the plugin.

The problem with that is that we can only naively export statics, as they are the only things we know where to look for. But when we are looking for a whatever structure there is that implements some trait, we cant just look in the object code and find it there, neither can we return such structure with a static function, because of it's return type being literally "whatever implements this trait".

So how we solve this problem is by saying: ok, the function can't return some non static stuff we want to export, but the thing is -- it can get whatever we want as an argument, even just some whatever trait implementor.

So what if our function actually gets a reference to a structure, that implements a trait, that allows us to place in it the implementor of our plugin behavior trait.

Basically the function we export basically is our proxy for the structure we want to export, as it both has all the object bindings to get the trait implementor from the object file, and has definition that is defined in such way that we can just easily find it inside of the object file when we are loading the plugin.

------------

So what we actually need to define in our api are two traits: one for the plugin interface, and one that loads an implementor of that trait into the outside program. Also we need a macro that will unify the way the static variable is exported, so that it can be imported without any problems, and also this variable should contain all the information needed for the version compatibility checks.

------------

**#2 Defining plugin importer structure**

As said in the previous part, for the trait implementor to be exported, it should be placed inside of some other object from the outside. We have already defined the way this structure would work when we defined the registrar trait, but to make usage of plugin even simpler, so that each program that uses the plugins does not need to make it's own registrar.

The structure we create has to have some fields, containing plugin name and version info, as well as the field in which a common behavior trait implementor should be placed. Also, for the library not to be deleted from the memory before we get rid of the trait implementor inside of it, we need to make sure, an RC copy of the library is always carried with that implementor when we call it. So to do this we can just add a library field to the registrar structure and also implement the common behavior trait on it in such way that it just pipes all the arguments and returns through itself to the inside trait implementor field, so that we don't always need to call a function from this common behavior trait on the structure's field, but just on the structure itself, and just use this structure as if it was the imported one, but having ability to lookup the name and version of the plugin and being sure the library it is inside of is not unloaded before the implementor in it is.

The plugin load function itself is not that hard -- it just loads the library from the path provided as an argument, RC copies it into the registrar, which the load function was called on (self), checks if all the versions match and runs imported static function on itself, as it implements the registrar trait.
(By the way the reason why the imported function has to call another function on the argument provided instead of just putting the common trait implementor right into the registrar, is because while the registrar is defined in the api, it may be completely ignored by the user, and swapped for the registrar the user decided to write on their own, so the function should take a trait implementor as an argument, and traits do not define fields, so the function is called instead)

------------

**#3 Defining common behavior**

The communication between the host and the plugin is done through functions, defined in the common behavior trait, but the problem with this function definition is that in such way we can only define a finite amount of functions with fixed arguments and returns, which means that to make our plugin to export multiple functions, it must just proxy the calls, received by some function from the common behavior (function "call"), get information about the function it needs to call by it's id and all arguments, it needs to pass to it.

To get rid of the limit on argument types and amounts, we will make this function to get it's arguments as a vector of Any trait implementors, which means, a list of any amount of any objects.

Also, we have to change return type to an any trait implementor, so that our function can return anything we may want.

(For efficiency reasons, instead of providing the function name to the call function, we provide it's id number, which allows to access this function easier when it's needed, as it may just be placed in some vector of functions)

For this to work properly, we need a function that will provide information about available functions, their ids, names, argument types and return types (get_function_list function)

Also, if some of our functions want to return some uncommon types of arguments, which we want the host later to forward back to one of our other functions, we must notify the host about this type through get_type_list function.

Some plugins need to do some things before they can be used. Such things should be placed inside of the init function, which must be called by the host after loading the plugin before it can be used.

As sometimes the plugin might need access to another plugin's functions to function, it can provide these plugins in the return of the init function. For this to work correctly, interplug_deny and interplug_provide functions should also be defined, so that the plugin can actually get RC copies of plugins it needs.
