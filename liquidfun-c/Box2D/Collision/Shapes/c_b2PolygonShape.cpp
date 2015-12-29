#include "c_b2PolygonShape.h"

extern "C" {

    b2PolygonShape* b2PolygonShape_New() {
        return new b2PolygonShape;
    }

    void b2PolygonShape_Delete(b2PolygonShape* self) {
        delete self;
    }

    const b2Vec2& b2PolygonShape_GetVertex(b2PolygonShape* self, int32 index) {
        return self->GetVertex(index);
    }

    int32 b2PolygonShape_GetVertexCount(const b2PolygonShape* self) {
        return self->GetVertexCount();
    }

    void b2PolygonShape_SetAsBox(b2PolygonShape* self, float32 hx, float32 hy) {
        self->SetAsBox(hx, hy);
    }

    void b2PolygonShape_SetAsBox_Oriented(b2PolygonShape* self, float32 hx, float32 hy, const b2Vec2& center, float32 angle) {
        self->SetAsBox(hx, hy, center, angle);
    }

    b2Shape* b2PolygonShape_Upcast(b2PolygonShape* self) {
        return static_cast<b2Shape*>(reinterpret_cast<b2PolygonShape*>(self));
    }

} // extern C