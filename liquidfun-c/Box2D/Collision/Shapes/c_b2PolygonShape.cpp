#include "c_b2PolygonShape.h"

extern "C" {

    b2PolygonShape* b2PolygonShape_New() {
        return new b2PolygonShape;
    }

    void b2PolygonShape_Delete(b2PolygonShape* self) {
        delete self;
    }

    void b2PolygonShape_SetAsBox(b2PolygonShape* self, float32 hx, float32 hy) {
        self->SetAsBox(hx, hy);
    }

    b2Shape* b2PolygonShape_Upcast(b2PolygonShape* s) {
        return static_cast<b2Shape*>(reinterpret_cast<b2PolygonShape*>(s));
    }

} // extern C