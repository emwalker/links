'use client'

import { Box, Button, Group, TextInput, Title } from '@mantine/core'
import { UseFormReturnType, useForm } from '@mantine/form'
import { useRouter } from 'next/navigation'
import { AppRouterInstance } from 'next/dist/shared/lib/app-router-context.shared-runtime'
import { notifications } from '@mantine/notifications'
import { Page } from '@/components/Page'
import { createUser } from '@/lib/store'
import classes from './page.module.css'

type Errors = {
  [key: string]: string
}

function submitForm(router: AppRouterInstance, form: UseFormReturnType<any>) {
  return form.onSubmit(async (payload) => {
    const res = await createUser(payload)
    const errorFields = Object.keys(res.errors)

    if (errorFields.length > 0) {
      const errors: Errors = {}

      errorFields.forEach((memo, fieldIndex) => {
        const field = errorFields[fieldIndex]
        const [first] = res.errors[field] || []
        if (first) {
          errors[field] = first
        }
        return memo
      }, {})

      form.setErrors(errors)
      return
    }

    const { name } = payload

    console.log('user added: ', res)
    router.push('/users')
    notifications.show({
      title: 'User added',
      message: `A new user has been added: ${name}`,
      color: 'blue',
      autoClose: 8000,
    })
  })
}

export default function GET() {
  const router = useRouter()

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: '',
      username: '',
    },

    validate: {
      name: (value) => value ? null : 'A name is required',
      username: (value) => value ? null : 'A username is required',
    },
  })

  return (
    <Page>
      <Title order={3}>Add a topic</Title>

      <Box maw={700} className={classes.form}>
        <form onSubmit={submitForm(router, form)}>
          <TextInput
            withAsterisk
            label="Name"
            placeholder="Name of new user"
            key={form.key('name')}
            {...form.getInputProps('name')}
          />

          <TextInput
            withAsterisk
            label="Username"
            placeholder="Username for new user"
            key={form.key('username')}
            {...form.getInputProps('username')}
          />

          <Group justify="flex-start" mt="md">
            <Button type="submit">Add</Button>
          </Group>
        </form>
      </Box>
    </Page>
  )
}
