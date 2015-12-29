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

    int32 b2World_GetJointCount(const b2World* self) {
        return self->GetJointCount();
    }

    const b2Body* b2World_GetBodyList(const b2World* self) {
        return self->GetBodyList();
    }

    c_b2Vec2 b2World_GetGravity(const b2World* self) {
    	b2Vec2 tmp = self->GetGravity();
        return *cast(&tmp);
    }

    b2ParticleSystem* b2World_GetParticleSystemList(b2World* self) {
        return self->GetParticleSystemList();
    }

    b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd) {
    	return self->CreateBody(bd);
    }

    b2RevoluteJoint* b2World_CreateRevoluteJoint(
        b2World* self,

        // b2JointDef
        const b2JointType type,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        const bool collideConnected,

        // b2RevoluteJointDef
        const b2Vec2 localAnchorA,
        const b2Vec2 localAnchorB,
        const float32 referenceAngle,
        const bool enableLimit,
        const float32 lowerAngle,
        const float32 upperAngle,
        const bool enableMotor,
        const float32 motorSpeed,
        const float32 maxMotorTorque
    ) {
        
        b2RevoluteJointDef jd;

        jd.type = type;
        jd.userData = userData;
        jd.bodyA = bodyA;
        jd.bodyB = bodyB;
        jd.collideConnected = collideConnected;
        jd.localAnchorA = localAnchorA;
        jd.localAnchorB = localAnchorB;
        jd.referenceAngle = referenceAngle;
        jd.enableLimit = enableLimit;
        jd.lowerAngle = lowerAngle;
        jd.upperAngle = upperAngle;
        jd.enableMotor = enableMotor;
        jd.motorSpeed = motorSpeed;
        jd.maxMotorTorque = maxMotorTorque;

        return (b2RevoluteJoint*)self->CreateJoint(&jd);
    }

    b2ParticleSystem* b2World_CreateParticleSystem(b2World* self, const b2ParticleSystemDef* def) {
        return self->CreateParticleSystem(def);
    }

    void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations) {
        self->Step(timeStep, velocityIterations, positionIterations);
    }


} // extern C

