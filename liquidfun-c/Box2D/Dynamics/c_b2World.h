#ifndef C_BOX2D_WORLD
#define C_BOX2D_WORLD

#ifdef __cplusplus
extern "C" {
#endif

	b2World* b2World_New(const b2Vec2* gravity);
	void b2World_Delete(b2World* self);
	c_b2Vec2 b2World_GetGravity(const b2World* self);
	b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd);
	void b2World_Step(box2d_World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations);

#ifdef __cplusplus
} // extern C
#endif
#endif