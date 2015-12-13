#ifndef C_B2_WORLD
#define C_B2_WORLD

#ifdef __cplusplus
extern "C" {
#endif

	b2World* b2World_New(const b2Vec2* gravity);
	void b2World_Delete(b2World* self);
	int32 b2World_GetBodyCount(const b2World* self);
	const b2Body* b2World_GetBodyList(const b2World* self);
	c_b2Vec2 b2World_GetGravity(const b2World* self);
	b2ParticleSystem* b2World_GetParticleSystemList(b2World* self);
	b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd);
	b2ParticleSystem* b2World_CreateParticleSystem(b2World* self, const b2ParticleSystemDef* def);
	void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations);

#ifdef __cplusplus
} // extern C
#endif
#endif