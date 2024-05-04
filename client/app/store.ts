export const ROOT_USER_ID = '2db58326-ddfa-4561-9ae2-232aa5c32277'

export type ErrorMap = { [key:string]: string[] }

export type User = {
  id: string,
  username: string,
  name: string | null,
  is_admin: boolean,
}

export type FetchUsersResponse = {
  total: number,
  items: User[],
  page: number,
}

export async function fetchUsers(): Promise<FetchUsersResponse> {
  const res = await fetch('http://localhost:3000/api/users', { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch users')
  }

  return res.json()
}

export type FetchUserResponse = {
  user: User | null,
}

export async function fetchUser(userId: string): Promise<FetchUserResponse> {
  const res = await fetch(`http://localhost:3000/api/users/${userId}`, { cache: 'no-cache' })

  if (!res.ok) {
    console.log('failed to fetch user: ', res)
  }

  return res.json()
}

export type Topic = {
  id: string,
  name: string,
}

export type FetchTopicsResponse = {
  total: number,
  items: Topic[],
  page: number,
}

export async function fetchTopics(): Promise<FetchTopicsResponse> {
  const res = await fetch('http://localhost:3000/api/topics', { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch topics')
  }

  return res.json()
}

export type FetchTopicResponse = {
  topic: Topic | null,
}

export async function fetchTopic(topicId: string): Promise<FetchTopicResponse> {
  const res = await fetch(`http://localhost:3000/api/topics/${topicId}`, { cache: 'no-cache' })

  if (!res.ok) {
    console.log('failed to fetch topic: ', res)
  }

  return res.json()
}

type CreateTopicPayload = {
  owner_id: string,
  name: string,
}

type CreateTopicResponse = {
  topic_id: string | null,
  errors: ErrorMap,
  created: boolean,
}

export async function createTopic(payload: CreateTopicPayload): Promise<CreateTopicResponse> {
  const res = await fetch('/api/topics', {
    headers: { 'Content-Type': 'application/json' },
    method: 'POST',
    body: JSON.stringify(payload),
  })

  if (!res.ok) {
    console.log('failed to create topic: ', res)
  }

  return res.json()
}
