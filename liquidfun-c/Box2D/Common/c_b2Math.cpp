#include "c_b2Math.h"

c_b2Vec2* cast(b2Vec2* v) {
    return reinterpret_cast<c_b2Vec2*>(v);
}
