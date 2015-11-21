#include <Box2D/Box2D.h>
#include "../Common/c_b2Math.h"
#include "c_b2World.h"

extern "C" {

    b2World* b2World_New(const b2Vec2* gravity) {
        return new b2World(*gravity);
    }

    void b2World_Delete(b2World* self) {
        delete self;
    }
    
    int32 b2World_GetBodyCount(const b2World* self) {
        return self->GetBodyCount();
    }

    const b2Body* b2World_GetBodyList(const b2World* self) {
        return self->GetBodyList();
    }

    c_b2Vec2 b2World_GetGravity(const b2World* self) {
    	b2Vec2 tmp = self->GetGravity();
        return *cast(&tmp);
    }

    b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd) {
    	return self->CreateBody(bd);
    }

    void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations) {
        self->Step(timeStep, velocityIterations, positionIterations);
    }


} // extern C

