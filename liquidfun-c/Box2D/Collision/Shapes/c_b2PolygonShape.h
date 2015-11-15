#ifndef C_BOX2D_POLYGONSHAPE
#define C_BOX2D_POLYGONSHAPE

#ifdef __cplusplus
extern "C" {
#endif

	b2PolygonShape* b2PolygonShape_New();
	void b2PolygonShape_Delete(b2PolygonShape* self);
	void b2PolygonShape_SetAsBox(b2PolygonShape* self, float32 hx, float32 hy);
	b2Shape* b2PolygonShape_Upcast(b2PolygonShape* s);

#ifdef __cplusplus
} // extern C
#endif

#endif
