DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/ ../rust-bindgen/target/debug/bindgen -l dispatch -match dispatch_ -o ./libdispatch.rs -match base.h -match block.h -match data.h -match dispatch.h -match group.h -match introspection.h -match io.h -match object.h -match once.h -match queue.h -match semaphore.h -match source.h -match time.h /usr/include/dispatch/dispatch.h