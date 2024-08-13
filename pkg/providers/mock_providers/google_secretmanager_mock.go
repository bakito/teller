// Code generated by MockGen. DO NOT EDIT.
// Source: pkg/providers/google_secretmanager.go
//
// Generated by this command:
//
//	mockgen -source pkg/providers/google_secretmanager.go -destination pkg/providers/mock_providers/google_secretmanager_mock.go
//

// Package mock_providers is a generated GoMock package.
package mock_providers

import (
	context "context"
	reflect "reflect"

	apiv1 "cloud.google.com/go/secretmanager/apiv1"
	v2 "github.com/googleapis/gax-go/v2"
	gomock "go.uber.org/mock/gomock"
	v1 "google.golang.org/genproto/googleapis/cloud/secretmanager/v1"
)

// MockGoogleSMClient is a mock of GoogleSMClient interface.
type MockGoogleSMClient struct {
	ctrl     *gomock.Controller
	recorder *MockGoogleSMClientMockRecorder
}

// MockGoogleSMClientMockRecorder is the mock recorder for MockGoogleSMClient.
type MockGoogleSMClientMockRecorder struct {
	mock *MockGoogleSMClient
}

// NewMockGoogleSMClient creates a new mock instance.
func NewMockGoogleSMClient(ctrl *gomock.Controller) *MockGoogleSMClient {
	mock := &MockGoogleSMClient{ctrl: ctrl}
	mock.recorder = &MockGoogleSMClientMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use.
func (m *MockGoogleSMClient) EXPECT() *MockGoogleSMClientMockRecorder {
	return m.recorder
}

// AccessSecretVersion mocks base method.
func (m *MockGoogleSMClient) AccessSecretVersion(ctx context.Context, req *v1.AccessSecretVersionRequest, opts ...v2.CallOption) (*v1.AccessSecretVersionResponse, error) {
	m.ctrl.T.Helper()
	varargs := []any{ctx, req}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "AccessSecretVersion", varargs...)
	ret0, _ := ret[0].(*v1.AccessSecretVersionResponse)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// AccessSecretVersion indicates an expected call of AccessSecretVersion.
func (mr *MockGoogleSMClientMockRecorder) AccessSecretVersion(ctx, req any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, req}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "AccessSecretVersion", reflect.TypeOf((*MockGoogleSMClient)(nil).AccessSecretVersion), varargs...)
}

// AddSecretVersion mocks base method.
func (m *MockGoogleSMClient) AddSecretVersion(ctx context.Context, req *v1.AddSecretVersionRequest, opts ...v2.CallOption) (*v1.SecretVersion, error) {
	m.ctrl.T.Helper()
	varargs := []any{ctx, req}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "AddSecretVersion", varargs...)
	ret0, _ := ret[0].(*v1.SecretVersion)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// AddSecretVersion indicates an expected call of AddSecretVersion.
func (mr *MockGoogleSMClientMockRecorder) AddSecretVersion(ctx, req any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, req}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "AddSecretVersion", reflect.TypeOf((*MockGoogleSMClient)(nil).AddSecretVersion), varargs...)
}

// DestroySecretVersion mocks base method.
func (m *MockGoogleSMClient) DestroySecretVersion(ctx context.Context, req *v1.DestroySecretVersionRequest, opts ...v2.CallOption) (*v1.SecretVersion, error) {
	m.ctrl.T.Helper()
	varargs := []any{ctx, req}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "DestroySecretVersion", varargs...)
	ret0, _ := ret[0].(*v1.SecretVersion)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// DestroySecretVersion indicates an expected call of DestroySecretVersion.
func (mr *MockGoogleSMClientMockRecorder) DestroySecretVersion(ctx, req any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, req}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "DestroySecretVersion", reflect.TypeOf((*MockGoogleSMClient)(nil).DestroySecretVersion), varargs...)
}

// ListSecrets mocks base method.
func (m *MockGoogleSMClient) ListSecrets(ctx context.Context, in *v1.ListSecretsRequest, opts ...v2.CallOption) *apiv1.SecretIterator {
	m.ctrl.T.Helper()
	varargs := []any{ctx, in}
	for _, a := range opts {
		varargs = append(varargs, a)
	}
	ret := m.ctrl.Call(m, "ListSecrets", varargs...)
	ret0, _ := ret[0].(*apiv1.SecretIterator)
	return ret0
}

// ListSecrets indicates an expected call of ListSecrets.
func (mr *MockGoogleSMClientMockRecorder) ListSecrets(ctx, in any, opts ...any) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	varargs := append([]any{ctx, in}, opts...)
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "ListSecrets", reflect.TypeOf((*MockGoogleSMClient)(nil).ListSecrets), varargs...)
}
