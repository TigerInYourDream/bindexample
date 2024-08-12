#ifndef MYCLASSWRAPPER_H
#define MYCLASSWRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MyClassOpaque MyClassOpaque;
typedef MyClassOpaque* MyClassHandle;

MyClassHandle MyClass_create();
void MyClass_destroy(MyClassHandle handle);
void MyClass_myMethod(MyClassHandle handle);

#ifdef __cplusplus
}
#endif

#endif // MYCLASSWRAPPER_H