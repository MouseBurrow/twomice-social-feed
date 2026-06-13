# Social Feed Service API Reference

Externally via gateway at `/api/users/me/*`.  
Full URL: `https://host/api/users/me/following` → gateway → `GET /users/me/following`

---

## Auth

Most endpoints require `X-User-Id` header (gateway injects from session).  
Internal endpoints use query params instead.

---

## Error format

```json
{ "error": "error_code", "message": "Human readable message" }
```

| Status | error_code | Meaning |
|---|---|---|
| 401 | `Unauthorized` | Authentication required |
| 404 | `BoardNotFound` | Board does not exist |
| 409 | `UniqueViolation` | Already following this board |
| 502 | `UpstreamError` | Failed to reach post service |

---

## Endpoints

### `GET /users/me/following` — List boards I follow (protected)

Response `200`:
```json
[
  {
    "id": "general",
    "name": "general",
    "description": "General discussion",
    "post_count": 42
  },
  {
    "id": "tech",
    "name": "tech",
    "description": "Technology",
    "post_count": 17
  }
]
```

Enriched with live data from post service's active boards. Only active boards appear.

---

### `PUT /users/me/following/:board_id` — Follow a board (protected)

No body. Path param `:board_id` is the board name.

Response `204` No Content. Idempotent (`ON CONFLICT DO NOTHING`).

---

### `DELETE /users/me/following/:board_id` — Unfollow a board (protected)

No body. Path param `:board_id` is the board name.

Response `204` No Content.

---

### `GET /users/me/stats` — Get my profile stats (protected)

Aggregates data from post service and local follow count.

Response `200`:
```json
{
  "nib_count": 5,
  "squeak_count": 12,
  "upvote_count": 34,
  "following_count": 3
}
```

| Field | Source | Description |
|---|---|---|
| `nib_count` | Post service `/internal/stats/:user_id` | Non-deleted posts |
| `squeak_count` | Post service `/internal/stats/:user_id` | Non-deleted comments |
| `upvote_count` | Post service `/internal/stats/:user_id` | Net upvotes received |
| `following_count` | Local DB | Number of boards followed |

---

### `GET /internal/following?user_id=N` — Get follow list for any user (internal, no auth)

Query param: `user_id` (required, i64)

Response `200`: Same `FollowedBoardInfo` array as the authenticated endpoint.  
Not exposed through the gateway — used by other internal services.
