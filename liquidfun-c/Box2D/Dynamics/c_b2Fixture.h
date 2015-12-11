#ifndef C_B2_FIXTURE
#define C_B2_FIXTURE

#ifdef __cplusplus
extern "C" {
#endif

	b2Fixture* b2Fixture_GetNext(b2Fixture* self);
	b2Shape* b2Fixture_GetShape(b2Fixture* self);
	b2Shape::Type b2Fixture_GetType(b2Fixture* self);

#ifdef __cplusplus
} // extern C
#endif
#endif