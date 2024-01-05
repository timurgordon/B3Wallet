import { createReActor } from "@ic-reactor/react"
import { ReActorMethodField } from "@ic-reactor/store"
import { b3wallet, canisterId, idlFactory } from "@src/declarations/b3wallet"

export type B3Wallet = typeof b3wallet

export const {
  useQueryCall: useWalletQuery,
  useUpdateCall: useWalletUpdate,
  useAuthClient: useWalletAuthClient,
  useActorStore: useWalletActorStore,
  useMethodFields: useWalletMethodFields
} = createReActor<B3Wallet>({
  canisterId,
  idlFactory,
  withDevtools: true,
  host: "https://ic0.app"
})

export type WalletDynamicField = ReActorMethodField<B3Wallet>
