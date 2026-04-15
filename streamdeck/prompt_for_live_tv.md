# Instructions for AI Agent: Implement Dynamic "Live TV" (Sports) Section

**Context:**
I have an Electron/Tauri desktop streaming dashboard app. Currently, the "Live TV" section (Cricket, Football, Motorsports) uses hardcoded static data (`SPORTS_DATA` in `src/app.js`). I want you to make this section fully dynamic by fetching real-time match data from free sports APIs.

**Goal:**
Replace the static `SPORTS_DATA` with live data feeds showing current and upcoming matches, updating the UI accordingly.

**Files to Modify:**
1.  `src/app.js` (JavaScript logic)
2.  `src/index.html` (Settings UI for API keys)

---

### Step 1: Add API Key Configuration (UI & Storage)
We need a way for the user to input their sports API keys, similar to how the TMDB API key is handled.

1.  **In `src/index.html`**: Add input fields for Sports API keys to the existing settings/overlays (e.g., inside the `<div id="api-key-overlay">` or a new settings panel).
    *   Input for "API-Football Key"
    *   Input for "Cricket API Key" (e.g., CricAPI or Sportmonks)
2.  **In `src/app.js`**:
    *   Create variables to retrieve these keys from `localStorage` (e.g., `localStorage.getItem('api_football_key')`).
    *   Update the save/skip functions to handle saving these new keys to local storage alongside the TMDB key.

### Step 2: Choose and Setup Free APIs
Implement fetch logic using free tiers of the following (or similar) APIs:
1.  **Football / General Sports:** Use **API-Football** (or API-Sports.io). Their free tier allows 100 requests/day and provides excellent live scores.
2.  **Cricket:** Use **CricketData.org** (Free tier available) or **Sportmonks** (Free tier for basic endpoints).

### Step 3: Replace Static `SPORTS_DATA` in `app.js`
1.  Locate the hardcoded `const SPORTS_DATA = { ... }` object around line 1450 in `src/app.js`.
2.  Remove or comment out this static object.
3.  Create an asynchronous function `fetchLiveSportsData()` that polls your chosen APIs.
    *   Fetch live and upcoming Cricket matches.
    *   Fetch live and upcoming Football matches (e.g., Premier League, La Liga).
    *   *Optional:* Fetch F1 data if an API is available (e.g., OpenF1.org), otherwise keep F1 static or remove it.
4.  Map the raw API responses into an object structure that matches the original `SPORTS_DATA` format so the UI renders correctly:
    ```javascript
    {
       cricket: [{ id: 'api_id', title: 'Team A vs Team B', platform: 'hotstar', status: 'LIVE', time: '...', thumb: '...' }],
       football: [...]
    }
    ```

### Step 4: Update `renderSportsHub()`
1.  Modify the existing `renderSportsHub()` function in `src/app.js` so that it calls your new `fetchLiveSportsData()` function first.
2.  Ensure it gracefully handles missing API keys (e.g., by showing a "Configure API Key" message in the UI or falling back to empty rows).
3.  Keep the existing DOM creation logic (`document.createElement('div')`, `.className = 'sports-card'`, etc.) but feed it the dynamic data.

### Step 5: Implementation Details & Best Practices
*   **Rate Limiting:** Because we are using free API tiers, implement a simple caching mechanism or ensure you only fetch data once every few minutes (e.g., `setInterval` for 5 minutes), rather than on every render.
*   **Error Handling:** Wrap all API `fetch` calls in `try-catch` blocks. If an API fails or the rate limit is exceeded, fail gracefully without breaking the rest of the application.
*   **Thumbnails:** API responses might not include high-quality thumbnails. You may need to use fallback generic sports images (like the Unsplash links currently in the code) if specific match images aren't provided by the API.

**Output required:**
Please provide the exact code changes required for `src/index.html` and `src/app.js` to accomplish this.
