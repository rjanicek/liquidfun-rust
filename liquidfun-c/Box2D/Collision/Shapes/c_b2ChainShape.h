#ifndef C_BOX2D_CHAINSHAPE
#define C_BOX2D_CHAINSHAPE

#ifdef __cplusplus
extern "C" {
#endif

	b2ChainShape* b2ChainShape_New();
	void b2ChainShape_Delete(b2ChainShape* self);
	b2Shape* b2ChainShape_Upcast(b2ChainShape* self);
	void b2ChainShape_CreateChain(b2ChainShape* self, const b2Vec2* vertices, int32 count);
	int32 b2ChainShape_GetChildCount(b2ChainShape* self);
	int32 b2ChainShape_m_count(b2ChainShape* self);
	b2Vec2* b2ChainShape_m_vertices(b2ChainShape* self);

#ifdef __cplusplus
} // extern C
#endif

#endif
