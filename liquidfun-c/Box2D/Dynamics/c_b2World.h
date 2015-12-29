#ifndef C_B2_WORLD
#define C_B2_WORLD

#ifdef __cplusplus
extern "C" {
#endif

	b2World* b2World_New(const b2Vec2* gravity);
	void b2World_Delete(b2World* self);
	int32 b2World_GetBodyCount(const b2World* self);
	int32 b2World_GetJointCount(const b2World* self);
	const b2Body* b2World_GetBodyList(const b2World* self);
	c_b2Vec2 b2World_GetGravity(const b2World* self);
	b2ParticleSystem* b2World_GetParticleSystemList(b2World* self);
	b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd);

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
	);

	b2ParticleSystem* b2World_CreateParticleSystem(b2World* self, const b2ParticleSystemDef* def);
	void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations);

#ifdef __cplusplus
} // extern C
#endif
#endif