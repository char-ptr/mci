# mci
minecraft cheat interface

## why

basically i got bored of having to do shit with clients and then manually writing classes to interact with jvm in c++,
so with this project i decided to just have it automatically generate all the required mappings to every minecraft class

## how to use
i don't recommend using right now.

## progress 
### jni wrapper
- [x] jclass / jobject
  - [x] get fields
  - [x] set fields
  - [x] get static fields
  - [x] set static fields
  - [x] call methods
  - [x] call static methods
- [ ] general QOL
  - [x] jclass not operable with jobject (prevents you from shooting yourself)
  - [ ] easier setting & getting via traits/macros (?)
- [ ] types
  - [x] jclass
  - [x] jobject
  - [x] jvalue
  - [x] env (i)
  - [ ] jarray (ii)
  - [ ] jstring (ii)
  - [ ] others...
  #### information
  1. pretty much bare essentials like find class have wrapped values
  2. working on, should be done soon
  
  ### bindings / mapping generator
  - [x] map types
    - [x] tiny
    - [x] yarn
  - [ ] generated output
    - [x] classes / modules
    - [x] get fields
    - [x] get static fields
    - [x] call methods
    - [x] call static methods
    - [ ] set fields
    - [ ] set fields
    - [ ] compilable 
    - [ ] auto static detection (i)
 #### information
 1. im currently working on a tool to get static fields and method names from a jar file.
 ### MCI (root)
 im not even working on it until mappings work lol
