# TODO — Social-Feed Service API Fixes

## `/users/me/following` — Return enriched board metadata

### Files:
- `services/social-feed/src/routes/following.rs` (route handler)
- `services/social-feed/src/service.rs` (data fetching)

### Current behavior:
Returns `Vec<String>` — just an array of board names:
```json
["technology", "art", "music"]
```

### Required behavior:
Returns an array of objects with board metadata:
```json
[
  { "id": "technology", "name": "Technology", "description": "...", "post_count": 42 },
  { "id": "art", "name": "Art & Design", "description": "...", "post_count": 18 }
]
```

### Frontend type (`FollowedBoardInfo`):
```ts
type FollowedBoardInfo = {
    id: string;
    name: string;
    description: string;
    post_count: number;
};
```

### Implementation plan:

1. **Define a new struct** in `service.rs`:
   ```rust
   #[derive(Serialize)]
   pub struct FollowedBoardInfo {
       pub id: String,
       pub name: String,
       pub description: String,
       pub post_count: i64,
   }
   ```

2. **Update `get_followed_boards()`** in `service.rs`:
   - For each followed board name, fetch board metadata from the post service
   - Options:
     a. Call the post service internally via `GET /internal/board/:name` for each board
     b. Call `GET /internal/boards` once to get all boards, then filter and enrich
   - Join post count data (available from the post service's stats or active boards endpoint)

3. **Update route handler** in `routes/following.rs`:
   - Change return type from `Json<Vec<String>>` to `Json<Vec<FollowedBoardInfo>>`
   - Same change for `internal_get_following` if used by other services

### Why:
The frontend profile page now shows rich following cards with board display name,
description, and post count (matching the prototype design). Previously only showed
bare board slugs like "b/technology" with no context.
