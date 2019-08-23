#include <node.h>

extern "C" {
	extern int32_t fibonacci(int32_t input);
}

namespace demo {

using v8::FunctionCallbackInfo;
using v8::Isolate;
using v8::Local;
using v8::NewStringType;
using v8::Object;
using v8::String;
using v8::Value;
using v8::Number;
using v8::Exception;


void Method(const FunctionCallbackInfo<Value>& args) {
  Isolate* isolate = args.GetIsolate();
  if (args.Length() != 1) {
    // Throw an Error that is passed back to JavaScript
    isolate->ThrowException(Exception::TypeError(
      String::NewFromUtf8(
        isolate, "Wrong number of arguments",
        NewStringType::kNormal
      ).ToLocalChecked()));
    return;
  }

  if (!args[0]->IsNumber()) {
    isolate->ThrowException(Exception::TypeError(
        String::NewFromUtf8(
          isolate,
          "Wrong argument type",
          NewStringType::kNormal
        ).ToLocalChecked()));
    return;
  }

  int value = args[0].As<Number>()->Value();
  args.GetReturnValue().Set(Number::New(isolate, fibonacci(value)));
}

void Initialize(Local<Object> exports) {
  NODE_SET_METHOD(exports, "fibonacci", Method);
}

  NODE_MODULE(NODE_GYP_MODULE_NAME, Initialize)
}
