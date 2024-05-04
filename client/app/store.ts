export type User = {
  id: string,
  username: string,
  name: String,
  is_admin: boolean,
}

export type FetchUsersResponse = {
  total: Number,
  items: User[],
  page: Number,
}

export async function fetchUsers(): Promise<FetchUsersResponse> {
  const res = await fetch('http://localhost:3000/api/users', { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch users')
  }

  return res.json()
}

export type Topic = {
  id: string,
  name: string,
}

export type FetchTopicsResponse = {
  total: Number,
  items: Topic[],
  page: Number,
}

export async function fetchTopics(): Promise<FetchTopicsResponse> {
  const res = await fetch('http://localhost:3000/api/topics', { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch topics')
  }

  return res.json()
}
