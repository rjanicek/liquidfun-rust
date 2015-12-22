#include "c_b2ChainShape.h"

extern "C" {

    b2ChainShape* b2ChainShape_New() {
        return new b2ChainShape;
    }

    void b2ChainShape_Delete(b2ChainShape* self) {
        delete self;
    }

    b2Shape* b2ChainShape_Upcast(b2ChainShape* self) {
        return static_cast<b2Shape*>(reinterpret_cast<b2ChainShape*>(self));
    }

    void b2ChainShape_CreateChain(b2ChainShape* self, const b2Vec2* vertices, int32 count) {
        self->CreateChain(vertices, count);
    }

    int32 b2ChainShape_GetChildCount(b2ChainShape* self) {
        return self->GetChildCount();
    }

    int32 b2ChainShape_m_count(b2ChainShape* self) {
        return self->m_count;
    }

    b2Vec2* b2ChainShape_m_vertices(b2ChainShape* self) {
        return self->m_vertices;
    }

} // extern C